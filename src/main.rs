mod constants;
mod game_object;
mod level;
mod utils;

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use bevy::prelude::*;
use constants::GRID_SIZE;
use game_object::{
    spawn_object_of_type, Animatable, Deadly, Exit, Floatable, GameObjectAssets, Liquid, Massive,
    Movable, ObjectType, Player, Position,
};
use level::{Dimensions, Level, LEVELS};
use rand::{thread_rng, Rng};
use utils::load_asset;

#[derive(Component)]
struct Background;

#[derive(Resource)]
struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

impl Deref for AnimationTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AnimationTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Resource)]
struct CurrentLevel {
    level: usize,
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
        .add_event::<LevelEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, on_keyboard_input)
        .add_systems(
            Update,
            (
                on_level_event,
                run_animations,
                check_for_deadly,
                check_for_exit,
                check_for_liquid,
            )
                .after(on_keyboard_input),
        )
        .add_systems(Update, position_entities.after(on_level_event))
        .run();
}

#[allow(clippy::type_complexity)]
fn on_keyboard_input(
    mut player_query: Query<&mut Position, With<Player>>,
    mut objects_query: Query<(&mut Position, Option<&Movable>, Option<&Massive>), Without<Player>>,
    mut app_exit_events: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
    mut level_events: EventWriter<LevelEvent>,
    dimensions: Res<Dimensions>,
) {
    let mut move_player = |dx, dy| {
        'players: for mut player_position in &mut player_query {
            let new_x = player_position.x + dx;
            let new_y = player_position.y + dy;
            if new_x < 1 || new_x > dimensions.width || new_y < 1 || new_y > dimensions.height {
                continue;
            }

            let mut possible_collission_objects: Vec<_> = objects_query
                .iter_mut()
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

            possible_collission_objects.sort_unstable_by_key(|(position, ..)| {
                (position.x - new_x).abs() + (position.y - new_y).abs()
            });

            let can_push_movable_to = |x: i16, y: i16| -> bool {
                for (position, movable, massive) in &possible_collission_objects {
                    let has_target_position = position.x == x && position.y == y;
                    let can_push_to = !movable.is_some() && !massive.is_some();
                    if has_target_position && !can_push_to {
                        return false;
                    }
                }
                true
            };

            let mut moved_object_indices = Vec::new();
            for (index, (position, movable, massive)) in
                possible_collission_objects.iter().enumerate()
            {
                if position.x == new_x && position.y == new_y {
                    if movable.is_some() && can_push_movable_to(new_x + dx, new_y + dy) {
                        moved_object_indices.push(index);
                        continue;
                    }

                    if massive.is_some() {
                        continue 'players;
                    }
                }
            }

            for index in moved_object_indices {
                let position = &mut possible_collission_objects[index].0;
                position.x += dx;
                position.y += dy;
            }

            player_position.x = new_x;
            player_position.y = new_y;
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

fn run_animations(
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

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut game_object_assets: ResMut<GameObjectAssets>,
    mut level_events: EventWriter<LevelEvent>,
) {
    commands.spawn(Camera2dBundle::default());

    let background_sprite = SpriteBundle {
        texture: images.add(load_asset(include_bytes!(
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

type LiquidSystemObject<'a> = (
    Entity,
    &'a Position,
    Option<&'a Liquid>,
    Option<&'a Floatable>,
    Option<&'a Player>,
);

fn check_for_liquid(
    mut commands: Commands,
    objects_query: Query<LiquidSystemObject>,
    mut level_events: EventWriter<LevelEvent>,
) {
    let (liquids, objects): (Vec<LiquidSystemObject>, Vec<LiquidSystemObject>) = objects_query
        .iter()
        .partition(|(_, _, liquid, ..)| liquid.is_some());

    for (_liquid, liquid_position, ..) in liquids {
        for (object, object_position, _, floatable, player) in &objects {
            if liquid_position == *object_position {
                if floatable.is_some() {
                    if !objects
                        .iter()
                        .any(|(other, other_position, _, floatable, _)| {
                            other != object
                                && other_position == object_position
                                && floatable.is_some()
                        })
                    {
                        let mut object = commands.entity(*object);
                        object.remove::<Movable>();
                    }
                } else if !objects.iter().any(|(_, other_position, _, floatable, _)| {
                    other_position == object_position && floatable.is_some()
                }) {
                    commands.entity(*object).despawn();

                    if player.is_some() {
                        level_events.send(LevelEvent::Reload);
                    }
                }
            }
        }
    }
}

fn on_level_event(
    commands: Commands,
    background_query: Query<Entity, With<Background>>,
    mut level_events: EventReader<LevelEvent>,
    mut current_level: ResMut<CurrentLevel>,
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
    objects: BTreeMap<ObjectType, Vec<Position>>,
    assets: &GameObjectAssets,
) {
    for (object_type, positions) in objects {
        for position in positions {
            spawn_object_of_type(commands, assets, object_type, position);
        }
    }
}
