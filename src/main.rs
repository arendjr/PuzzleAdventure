mod constants;
mod game_object;
mod level;
mod utils;

use std::collections::BTreeMap;

use bevy::prelude::*;
use constants::GRID_SIZE;
use game_object::{
    spawn_object_of_type, Exit, GameObjectAssets, Massive, Movable, ObjectType, Player, Position,
};
use level::{load_level, Dimensions, LEVELS};
use utils::load_asset;

#[derive(Component)]
struct Background;

#[derive(Default, Resource)]
struct CurrentLevel {
    level: usize,
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
        .init_resource::<Dimensions>()
        .init_resource::<CurrentLevel>()
        .init_resource::<GameObjectAssets>()
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, on_keyboard_input)
        .add_systems(Update, (position_entities, check_for_exit))
        .run();
}

#[allow(clippy::type_complexity)]
fn on_keyboard_input(
    mut player_query: Query<&mut Position, With<Player>>,
    mut objects_query: Query<(&mut Position, Option<&Movable>, Option<&Massive>), Without<Player>>,
    mut app_exit_events: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
    dimensions: Res<Dimensions>,
) {
    let mut move_player = |dx, dy| {
        'players: for mut player_position in &mut player_query {
            let new_x = player_position.x + dx;
            let new_y = player_position.y + dy;
            if new_x < 1 || new_x > dimensions.width || new_y < 1 || new_y > dimensions.height {
                continue;
            }

            for (position, movable, massive) in &mut objects_query {
                if position.x == new_x && position.y == new_y {
                    if movable.is_some() {
                        todo!();
                    }

                    if massive.is_some() {
                        continue 'players;
                    }
                }
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
            Escape => {
                app_exit_events.send(AppExit::Success);
                break;
            }
            _ => {}
        }
    }
}

fn position_entities(mut query: Query<(&Position, &mut Transform)>, dimensions: Res<Dimensions>) {
    for (position, mut transform) in &mut query {
        transform.translation.x =
            (-(dimensions.width * GRID_SIZE / 2) + position.x * GRID_SIZE - GRID_SIZE / 2) as f32;
        transform.translation.y =
            ((dimensions.height * GRID_SIZE / 2) - position.y * GRID_SIZE + GRID_SIZE / 2) as f32;
    }
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut dimensions: ResMut<Dimensions>,
    mut game_object_assets: ResMut<GameObjectAssets>,
) {
    commands.spawn(Camera2dBundle::default());

    let assets = GameObjectAssets::load(&mut images);
    let level = load_level(LEVELS[0]);

    let background_sprite = SpriteBundle {
        texture: images.add(load_asset(include_bytes!(
            "../assets/sprites/background.png"
        ))),
        ..Default::default()
    };
    commands
        .spawn((Background, background_sprite))
        .with_children(|cb| {
            spawn_level(cb, level.objects, &assets);
        });

    *dimensions.as_mut() = level.dimensions;
    *game_object_assets.as_mut() = assets;
}

fn check_for_exit(
    mut commands: Commands,
    background_query: Query<Entity, With<Background>>,
    player_query: Query<&Position, With<Player>>,
    exit_query: Query<&Position, With<Exit>>,
    mut current_level: ResMut<CurrentLevel>,
    mut dimensions: ResMut<Dimensions>,
    assets: Res<GameObjectAssets>,
) {
    for player_position in &player_query {
        for exit_position in &exit_query {
            if player_position == exit_position {
                current_level.level += 1;
                let level = load_level(LEVELS[current_level.level]);

                let background = background_query
                    .get_single()
                    .expect("there should be only one background");
                let mut background = commands.entity(background);
                background.despawn_descendants();
                background.with_children(|cb| {
                    spawn_level(cb, level.objects, &assets);
                });

                *dimensions.as_mut() = level.dimensions;
                return;
            }
        }
    }
}

fn spawn_level(
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
