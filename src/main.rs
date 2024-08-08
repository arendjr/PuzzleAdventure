mod constants;
mod errors;
mod game_object;
mod level;
mod timers;
mod utils;

use std::{cmp::Ordering, collections::BTreeMap};

use bevy::prelude::*;
use constants::GRID_SIZE;
use game_object::{
    spawn_object_of_type, Animatable, Deadly, Direction, Exit, ExplosionBundle, Explosive,
    Floatable, GameObjectAssets, Liquid, Massive, Movable, ObjectType, Openable, Player, Position,
    Pushable, SplashBundle, Trigger, Volatile,
};
use level::{Dimensions, InitialPositionAndDirection, Level, LEVELS};
use rand::{thread_rng, Rng};
use timers::{AnimationTimer, MovementTimer, TemporaryTimer};
use utils::load_repeating_asset;

#[derive(Component)]
struct Background;

#[derive(Default, Resource)]
struct CurrentLevel {
    level: usize,
}

#[derive(Default, Resource)]
struct PressedTriggers {
    num_pressed_triggers: usize,
}

#[derive(Event)]
enum LevelEvent {
    Advance,
    GoBack,
    Reload,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (768.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<AnimationTimer>()
        .init_resource::<CurrentLevel>()
        .init_resource::<Dimensions>()
        .init_resource::<GameObjectAssets>()
        .init_resource::<MovementTimer>()
        .init_resource::<PressedTriggers>()
        .init_resource::<TemporaryTimer>()
        .add_event::<LevelEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, on_keyboard_input)
        .add_systems(
            Update,
            (
                on_level_event,
                animate_objects,
                move_objects,
                check_for_deadly,
                check_for_exit,
                check_for_explosive,
                check_for_liquid,
                despawn_volatile_objects,
            )
                .after(on_keyboard_input),
        )
        .add_systems(
            Update,
            check_for_triggers
                .after(on_keyboard_input)
                .after(move_objects),
        )
        .add_systems(
            Update,
            position_entities
                .after(on_level_event)
                .after(check_for_explosive)
                .after(check_for_liquid),
        )
        .run();
}

#[allow(clippy::type_complexity)]
fn on_keyboard_input(
    mut player_query: Query<&mut Position, With<Player>>,
    mut collision_objects_query: Query<CollissionObject, Without<Player>>,
    mut app_exit_events: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
    mut level_events: EventWriter<LevelEvent>,
    dimensions: Res<Dimensions>,
) {
    let mut move_player = |dx, dy| {
        for mut player_position in &mut player_query {
            move_object(
                &mut player_position,
                (dx, dy),
                &dimensions,
                collision_objects_query.iter_mut(),
                true,
            );
        }
    };

    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            ArrowUp => move_player(0, -1),
            ArrowRight => move_player(1, 0),
            ArrowDown => move_player(0, 1),
            ArrowLeft => move_player(-1, 0),
            BracketRight => {
                level_events.send(LevelEvent::Advance);
            }
            BracketLeft => {
                level_events.send(LevelEvent::GoBack);
            }
            KeyR => {
                level_events.send(LevelEvent::Reload);
            }
            Escape => {
                app_exit_events.send(AppExit::Success);
                break;
            }
            _ => {}
        }
    }
}

fn position_entities(mut query: Query<(&Position, &mut Transform)>, dimensions: Res<Dimensions>) {
    let half_grid_size = GRID_SIZE / 2;
    for (position, mut transform) in &mut query {
        let x =
            (-(dimensions.width * half_grid_size) + position.x * GRID_SIZE - half_grid_size) as f32;
        if transform.translation.x != x {
            transform.translation.x = x;
        }
        let y =
            ((dimensions.height * half_grid_size) - position.y * GRID_SIZE + half_grid_size) as f32;
        if transform.translation.y != y {
            transform.translation.y = y;
        }
    }
}

fn animate_objects(
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

fn move_objects(
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
    if timer.just_finished() {
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
}

type CollissionObject<'a> = (Mut<'a, Position>, Option<&'a Pushable>, Option<&'a Massive>);

fn move_object<'a>(
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

fn despawn_volatile_objects(
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

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut game_object_assets: ResMut<GameObjectAssets>,
    mut level_events: EventWriter<LevelEvent>,
) {
    commands.spawn(Camera2dBundle::default());

    let background_sprite = SpriteBundle {
        texture: images.add(load_repeating_asset(include_bytes!(
            "../assets/sprites/background.png"
        ))),
        ..Default::default()
    };
    commands.spawn((Background, background_sprite));

    *game_object_assets.as_mut() = GameObjectAssets::load(&mut images, &mut texture_atlas_layouts);

    level_events.send(LevelEvent::Reload);
}

fn check_for_deadly(
    player_query: Query<&Position, With<Player>>,
    deadly_query: Query<&Position, With<Deadly>>,
    mut level_events: EventWriter<LevelEvent>,
) {
    for player_position in &player_query {
        for deadly_position in &deadly_query {
            if player_position == deadly_position {
                level_events.send(LevelEvent::Reload);
                return;
            }
        }
    }
}

fn check_for_exit(
    player_query: Query<&Position, With<Player>>,
    exit_query: Query<&Position, With<Exit>>,
    mut level_events: EventWriter<LevelEvent>,
) {
    for player_position in &player_query {
        for exit_position in &exit_query {
            if player_position == exit_position {
                level_events.send(LevelEvent::Advance);
                return;
            }
        }
    }
}

type ExplosiveSystemObject<'a> = (
    Entity,
    &'a Position,
    Option<&'a Explosive>,
    Option<&'a Player>,
);

fn check_for_explosive(
    mut commands: Commands,
    explosive_query: Query<ExplosiveSystemObject>,
    background_query: Query<Entity, With<Background>>,
    mut level_events: EventWriter<LevelEvent>,
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
                    level_events.send(LevelEvent::Reload);
                }
            }
        }
    }
}

type LiquidSystemObject<'a> = (
    Entity,
    &'a Position,
    Option<&'a Liquid>,
    Option<&'a Floatable>,
    Option<&'a Player>,
);

fn check_for_liquid(
    mut commands: Commands,
    liquid_query: Query<LiquidSystemObject>,
    background_query: Query<Entity, With<Background>>,
    mut level_events: EventWriter<LevelEvent>,
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
                        level_events.send(LevelEvent::Reload);
                    }
                }
            }
        }
    }
}

type TriggerSystemObject<'a> = (
    Entity,
    &'a Position,
    Option<&'a Openable>,
    Option<&'a Massive>,
    Option<&'a Trigger>,
    Option<&'a mut TextureAtlas>,
);

fn check_for_triggers(
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

fn on_level_event(
    commands: Commands,
    background_query: Query<Entity, With<Background>>,
    mut level_events: EventReader<LevelEvent>,
    mut current_level: ResMut<CurrentLevel>,
    mut pressed_triggers: ResMut<PressedTriggers>,
    dimensions: ResMut<Dimensions>,
    assets: Res<GameObjectAssets>,
) {
    let mut level = None;
    for event in level_events.read() {
        match event {
            LevelEvent::Advance => {
                if current_level.level < LEVELS.len() - 1 {
                    current_level.level += 1;
                    level = Some(current_level.level);
                }
            }
            LevelEvent::GoBack => {
                current_level.level = current_level.level.saturating_sub(1);
                level = Some(current_level.level);
            }
            LevelEvent::Reload => {
                level = Some(current_level.level);
            }
        }
    }

    if let Some(level) = level {
        load_level(commands, level, background_query, dimensions, assets);
        pressed_triggers.num_pressed_triggers = 0;
    }
}

fn load_level(
    mut commands: Commands,
    level: usize,
    background_query: Query<Entity, With<Background>>,
    mut dimensions: ResMut<Dimensions>,
    assets: Res<GameObjectAssets>,
) {
    let level = Level::load(LEVELS[level]);

    let background = background_query
        .get_single()
        .expect("there should be only one background");
    let mut background = commands.entity(background);
    background.despawn_descendants();
    background.with_children(|cb| {
        spawn_level_objects(cb, level.objects, &assets);
    });

    *dimensions.as_mut() = level.dimensions;
}

fn spawn_level_objects(
    commands: &mut ChildBuilder,
    objects: BTreeMap<ObjectType, Vec<InitialPositionAndDirection>>,
    assets: &GameObjectAssets,
) {
    for (object_type, initial_positions) in objects {
        for InitialPositionAndDirection {
            position,
            direction,
        } in initial_positions
        {
            spawn_object_of_type(
                commands,
                assets,
                object_type,
                position,
                direction.unwrap_or_default(),
            );
        }
    }
}
