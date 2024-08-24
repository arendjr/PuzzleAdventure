use std::cmp::Ordering;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    level::Dimensions,
    timers::{AnimationTimer, MovementTimer, TemporaryTimer},
    Background, GameEvent, PressedTriggers,
};

use super::{
    components::{Animatable, Direction, Trigger, *},
    object_bundles::*,
    GameObjectAssets,
};

pub fn animate_objects(
    mut timer: ResMut<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<(&Animatable, &mut TextureAtlas)>,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        for (animatable, mut atlas) in &mut query {
            atlas.index = thread_rng().gen_range(0..animatable.num_frames);
        }
    }
}

pub fn check_for_deadly(
    player_query: Query<&Position, With<Player>>,
    deadly_query: Query<&Position, With<Deadly>>,
    mut level_events: EventWriter<GameEvent>,
) {
    for player_position in &player_query {
        for deadly_position in &deadly_query {
            if player_position == deadly_position {
                level_events.send(GameEvent::LoadRelativeLevel(0));
                return;
            }
        }
    }
}

pub fn check_for_exit(
    player_query: Query<&Position, With<Player>>,
    exit_query: Query<&Position, With<Exit>>,
    mut level_events: EventWriter<GameEvent>,
) {
    for player_position in &player_query {
        for exit_position in &exit_query {
            if player_position == exit_position {
                level_events.send(GameEvent::LoadRelativeLevel(1));
                return;
            }
        }
    }
}

pub type ExplosiveSystemObject<'a> = (
    Entity,
    &'a Position,
    Option<&'a Explosive>,
    Option<&'a Player>,
);

pub fn check_for_explosive(
    mut commands: Commands,
    explosive_query: Query<ExplosiveSystemObject>,
    background_query: Query<Entity, With<Background>>,
    mut level_events: EventWriter<GameEvent>,
    mut temporary_timer: ResMut<TemporaryTimer>,
    assets: Res<GameObjectAssets>,
) {
    let (explosives, objects): (Vec<ExplosiveSystemObject>, Vec<ExplosiveSystemObject>) =
        explosive_query
            .iter()
            .partition(|(_, _, liquid, ..)| liquid.is_some());

    for (explosive, explosive_position, ..) in explosives {
        for (object, position, _, player) in &objects {
            if explosive_position == *position {
                commands.entity(explosive).despawn();
                commands.entity(*object).despawn();

                let background = background_query
                    .get_single()
                    .expect("there should be only one background");
                let mut background = commands.entity(background);
                background.with_children(|cb| {
                    cb.spawn(ExplosionBundle::spawn(&assets, **position));
                });
                if temporary_timer.finished() {
                    temporary_timer.reset();
                }

                if player.is_some() {
                    level_events.send(GameEvent::LoadRelativeLevel(0));
                }
            }
        }
    }
}

pub type LiquidSystemObject<'a> = (
    Entity,
    &'a Position,
    Option<&'a Liquid>,
    Option<&'a Floatable>,
    Option<&'a Player>,
);

pub fn check_for_liquid(
    mut commands: Commands,
    liquid_query: Query<LiquidSystemObject>,
    background_query: Query<Entity, With<Background>>,
    mut level_events: EventWriter<GameEvent>,
    mut temporary_timer: ResMut<TemporaryTimer>,
    assets: Res<GameObjectAssets>,
) {
    let (liquids, objects): (Vec<LiquidSystemObject>, Vec<LiquidSystemObject>) = liquid_query
        .iter()
        .partition(|(_, _, liquid, ..)| liquid.is_some());

    for (_liquid, liquid_position, ..) in liquids {
        for (object, position, _, floatable, player) in &objects {
            if liquid_position == *position {
                if floatable.is_some() {
                    if !objects
                        .iter()
                        .any(|(other, other_position, _, floatable, _)| {
                            other != object && other_position == position && floatable.is_some()
                        })
                    {
                        let mut object = commands.entity(*object);
                        object.remove::<Pushable>();
                    }
                } else if !objects.iter().any(|(_, other_position, _, floatable, _)| {
                    other_position == position && floatable.is_some()
                }) {
                    commands.entity(*object).despawn();

                    let background = background_query
                        .get_single()
                        .expect("there should be only one background");
                    let mut background = commands.entity(background);
                    background.with_children(|cb| {
                        cb.spawn(SplashBundle::spawn(&assets, **position));
                    });
                    if temporary_timer.finished() {
                        temporary_timer.reset();
                    }

                    if player.is_some() {
                        level_events.send(GameEvent::LoadRelativeLevel(0));
                    }
                }
            }
        }
    }
}

pub type TriggerSystemObject<'a> = (
    Entity,
    &'a Position,
    Option<&'a Openable>,
    Option<&'a Massive>,
    Option<&'a Trigger>,
    Option<&'a mut TextureAtlas>,
);

pub fn check_for_triggers(
    mut commands: Commands,
    mut query: Query<TriggerSystemObject>,
    mut pressed_triggers: ResMut<PressedTriggers>,
) {
    let mut triggers = Vec::new();
    let mut openables = Vec::new();
    let mut objects = Vec::new();
    for (entity, position, openable, massive, trigger, atlas) in &mut query {
        if trigger.is_some() {
            triggers.push(position);
        } else if openable.is_some() {
            openables.push((entity, massive, atlas));
        } else {
            objects.push(position);
        }
    }

    let num_pressed_triggers = triggers
        .iter()
        .filter(|trigger_position| objects.iter().any(|position| position == *trigger_position))
        .count();

    let opened = match num_pressed_triggers.cmp(&pressed_triggers.num_pressed_triggers) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => return, // No change.
    };

    for (openable, massive, atlas) in openables {
        if opened && massive.is_some() {
            commands.entity(openable).remove::<Massive>();

            if let Some(mut atlas) = atlas {
                atlas.index = 1;
            }
        } else if !opened && massive.is_none() {
            commands.entity(openable).insert(Massive);

            if let Some(mut atlas) = atlas {
                atlas.index = 0;
            }
        }
    }

    pressed_triggers.num_pressed_triggers = num_pressed_triggers;
}

pub fn despawn_volatile_objects(
    mut commands: Commands,
    mut timer: ResMut<TemporaryTimer>,
    time: Res<Time>,
    query: Query<Entity, With<Volatile>>,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}

pub type CollissionObject<'a> = (Mut<'a, Position>, Option<&'a Pushable>, Option<&'a Massive>);

pub fn move_objects(
    mut timer: ResMut<MovementTimer>,
    time: Res<Time>,
    mut movable_query: Query<(
        &mut Direction,
        &Movable,
        &mut Position,
        Option<&mut TextureAtlas>,
    )>,
    mut collision_objects_query: Query<CollissionObject, Without<Movable>>,
    dimensions: Res<Dimensions>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    for (mut direction, movable, mut position, mut atlas) in &mut movable_query {
        match movable {
            Movable::Bounce => {
                if !move_object(
                    &mut position,
                    direction.to_delta(),
                    &dimensions,
                    collision_objects_query.iter_mut(),
                    false,
                ) {
                    *direction = direction.inverse();
                }
            }
            Movable::FollowRightHand => {
                if move_object(
                    &mut position,
                    direction.right_hand().to_delta(),
                    &dimensions,
                    collision_objects_query.iter_mut(),
                    false,
                ) {
                    *direction = direction.right_hand();
                } else if !move_object(
                    &mut position,
                    direction.to_delta(),
                    &dimensions,
                    collision_objects_query.iter_mut(),
                    false,
                ) {
                    *direction = direction.left_hand();
                }
            }
        }

        if let Some(atlas) = atlas.as_mut() {
            atlas.index = *direction as usize;
        }
    }
}

pub fn move_object<'a>(
    position: &mut Mut<Position>,
    (dx, dy): (i16, i16),
    dimensions: &Dimensions,
    collission_objects: impl Iterator<Item = CollissionObject<'a>>,
    can_push: bool,
) -> bool {
    let new_x = position.x + dx;
    let new_y = position.y + dy;
    if new_x < 1 || new_x > dimensions.width || new_y < 1 || new_y > dimensions.height {
        return false;
    }

    let mut collission_objects: Vec<_> = collission_objects
        .filter(|(position, ..)| {
            if dx > 0 {
                position.x >= new_x && position.y == new_y
            } else if dx < 0 {
                position.x <= new_x && position.y == new_y
            } else if dy > 0 {
                position.x == new_x && position.y >= new_y
            } else if dy < 0 {
                position.x == new_x && position.y <= new_y
            } else {
                false
            }
        })
        .collect();

    collission_objects.sort_unstable_by_key(|(position, ..)| {
        (position.x - new_x).abs() + (position.y - new_y).abs()
    });

    let can_push_to = |x: i16, y: i16| -> bool {
        if x < 1 || x > dimensions.width || y < 1 || y > dimensions.height {
            return false;
        }
        for (position, pushable, massive) in &collission_objects {
            let has_target_position = position.x == x && position.y == y;
            let can_push_to = !pushable.is_some() && !massive.is_some();
            if has_target_position && !can_push_to {
                return false;
            }
        }
        true
    };

    let mut pushed_object_indices = Vec::new();
    for (index, (position, pushable, massive)) in collission_objects.iter().enumerate() {
        if position.x == new_x && position.y == new_y {
            if can_push && pushable.is_some() && can_push_to(new_x + dx, new_y + dy) {
                pushed_object_indices.push(index);
                continue;
            }

            if massive.is_some() {
                return false;
            }
        }
    }

    for index in pushed_object_indices {
        let position = &mut collission_objects[index].0;
        position.x += dx;
        position.y += dy;
    }

    position.x = new_x;
    position.y = new_y;
    true
}
