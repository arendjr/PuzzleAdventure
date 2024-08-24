mod constants;
mod editor;
mod errors;
mod fonts;
mod game_object;
mod level;
mod timers;
mod utils;

use std::{borrow::Cow, collections::BTreeMap, fs};

use bevy::{prelude::*, window::WindowResized};
use constants::{BACKGROUND_SIZE, EDITOR_WIDTH, GRID_SIZE, HALF_GRID_SIZE};
use editor::{spawn_selected_object, Editor, EditorBundle, EditorPlugin, SelectedObjectType};
use fonts::Fonts;
use game_object::{Direction, *};
use level::{Dimensions, InitialPositionAndDirection, Level, LEVELS};
use timers::{AnimationTimer, MovementTimer, TemporaryTimer};
use utils::{get_level_filename, load_repeating_asset};

#[derive(Component)]
struct Background;

#[derive(Resource)]
struct Levels {
    current_level: usize,
    levels: Vec<Cow<'static, str>>,
}

impl Default for Levels {
    fn default() -> Self {
        Self {
            current_level: 0,
            levels: LEVELS.iter().map(|c| (*c).into()).collect(),
        }
    }
}

#[derive(Default, Resource)]
struct PressedTriggers {
    num_pressed_triggers: usize,
}

#[derive(Resource)]
struct Zoom {
    factor: f32,
}

impl Default for Zoom {
    fn default() -> Self {
        Self { factor: 1.0 }
    }
}

#[derive(Event)]
enum GameEvent {
    ChangeWidth(i16),
    ChangeHeight(i16),
    ChangeZoom(f32),
    LoadRelativeLevel(isize),
    MovePlayer(i16, i16),
    ToggleEditor,
    Exit,
}

#[derive(Event)]
enum EditorEvent {
    Toggle,
}

#[derive(Event)]
enum SaveLevelEvent {
    Save,
}

#[derive(Event)]
enum TransformEvent {
    Update,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (BACKGROUND_SIZE, BACKGROUND_SIZE).into(),
                    ..default()
                }),
                ..default()
            }),
            EditorPlugin,
        ))
        .init_resource::<AnimationTimer>()
        .init_resource::<Dimensions>()
        .init_resource::<Fonts>()
        .init_resource::<GameObjectAssets>()
        .init_resource::<Levels>()
        .init_resource::<MovementTimer>()
        .init_resource::<PressedTriggers>()
        .init_resource::<TemporaryTimer>()
        .init_resource::<Zoom>()
        .add_event::<EditorEvent>()
        .add_event::<GameEvent>()
        .add_event::<SaveLevelEvent>()
        .add_event::<TransformEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (on_keyboard_input, on_resize_system, save_level))
        .add_systems(
            Update,
            (
                on_game_event,
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
        .add_systems(Update, load_level.after(on_game_event).after(save_level))
        .add_systems(
            Update,
            position_entities
                .after(load_level)
                .after(check_for_explosive)
                .after(check_for_liquid)
                .after(spawn_selected_object),
        )
        .add_systems(
            Update,
            (resize_background, toggle_editor)
                .after(load_level)
                .after(on_resize_system),
        )
        .add_systems(
            Update,
            update_level_transform
                .after(toggle_editor)
                .after(resize_background),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut font_assets: ResMut<Assets<Font>>,
    mut image_assets: ResMut<Assets<Image>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut game_object_assets: ResMut<GameObjectAssets>,
    mut level_events: EventWriter<GameEvent>,
    mut fonts: ResMut<Fonts>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Background,
        SpriteBundle {
            texture: image_assets.add(load_repeating_asset(include_bytes!(
                "../assets/sprites/background.png"
            ))),
            ..Default::default()
        },
        TextureAtlas {
            layout: Handle::default(),
            index: 0,
        },
    ));

    *game_object_assets.as_mut() =
        GameObjectAssets::load(&mut image_assets, &mut texture_atlas_layouts);

    fonts.poppins_light = font_assets.add(
        Font::try_from_bytes(Vec::from(include_bytes!(
            "../assets/font/Poppins/Poppins-Light.ttf"
        )))
        .unwrap(),
    );

    level_events.send(GameEvent::LoadRelativeLevel(0));
}

fn on_keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut events: EventWriter<GameEvent>) {
    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            ArrowUp => events.send(GameEvent::MovePlayer(0, -1)),
            ArrowRight => events.send(GameEvent::MovePlayer(1, 0)),
            ArrowDown => events.send(GameEvent::MovePlayer(0, 1)),
            ArrowLeft => events.send(GameEvent::MovePlayer(-1, 0)),
            Equal => events.send(GameEvent::ChangeZoom(1.25)),
            Minus => events.send(GameEvent::ChangeZoom(0.8)),
            BracketRight => events.send(GameEvent::LoadRelativeLevel(1)),
            BracketLeft => events.send(GameEvent::LoadRelativeLevel(-1)),
            KeyE => events.send(GameEvent::ToggleEditor),
            KeyR => events.send(GameEvent::LoadRelativeLevel(0)),
            Escape => events.send(GameEvent::Exit),
            _ => continue,
        };
    }
}

fn position_entities(
    mut query: Query<(&Position, &mut Transform), Changed<Position>>,
    dimensions: Res<Dimensions>,
) {
    for (position, mut transform) in &mut query {
        transform.translation.x =
            (-(dimensions.width * HALF_GRID_SIZE) + position.x * GRID_SIZE - HALF_GRID_SIZE) as f32;
        transform.translation.y =
            ((dimensions.height * HALF_GRID_SIZE) - position.y * GRID_SIZE + HALF_GRID_SIZE) as f32;
    }
}

#[allow(clippy::too_many_arguments)]
fn on_game_event(
    mut app_exit_events: EventWriter<AppExit>,
    mut collision_objects_query: Query<CollissionObject, Without<Player>>,
    mut dimensions: ResMut<Dimensions>,
    mut editor_events: EventWriter<EditorEvent>,
    mut level_events: EventReader<GameEvent>,
    mut levels: ResMut<Levels>,
    mut player_query: Query<&mut Position, With<Player>>,
    mut selected_object_type: ResMut<SelectedObjectType>,
    mut transform_events: EventWriter<TransformEvent>,
    mut zoom: ResMut<Zoom>,
    editor_query: Query<Entity, With<Editor>>,
) {
    for event in level_events.read() {
        match event {
            GameEvent::ChangeHeight(delta) => {
                if dimensions.height + delta > 0 {
                    dimensions.height += delta;
                }
            }
            GameEvent::ChangeWidth(delta) => {
                if dimensions.width + delta > 0 {
                    dimensions.width += delta;
                }
            }
            GameEvent::ChangeZoom(factor) => {
                zoom.factor *= factor;
                transform_events.send(TransformEvent::Update);
            }
            GameEvent::LoadRelativeLevel(delta) => {
                levels.current_level = (levels.current_level as isize + delta)
                    .clamp(0, levels.levels.len() as isize - 1)
                    as usize;
            }
            GameEvent::MovePlayer(dx, dy) => {
                let mut position = player_query
                    .get_single_mut()
                    .expect("there should be only one player");
                move_object(
                    &mut position,
                    (*dx, *dy),
                    &dimensions,
                    collision_objects_query.iter_mut(),
                    true,
                );
                transform_events.send(TransformEvent::Update);
            }
            GameEvent::ToggleEditor => {
                editor_events.send(EditorEvent::Toggle);
            }
            GameEvent::Exit => {
                if selected_object_type.is_some() {
                    **selected_object_type = None;
                } else if editor_query.get_single().is_ok() {
                    editor_events.send(EditorEvent::Toggle);
                } else {
                    app_exit_events.send(AppExit::Success);
                }
            }
        }
    }
}

fn resize_background(
    mut background_query: Query<&mut TextureAtlas, With<Background>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut transform_events: EventWriter<TransformEvent>,
    dimensions: Res<Dimensions>,
) {
    if !dimensions.is_changed() {
        return;
    }

    let mut background_atlas = background_query
        .get_single_mut()
        .expect("there should be only one background");

    let mut layout =
        TextureAtlasLayout::new_empty(UVec2::new(BACKGROUND_SIZE as u32, BACKGROUND_SIZE as u32));
    let x = (BACKGROUND_SIZE - dimensions.width * GRID_SIZE).clamp(0, BACKGROUND_SIZE) as u32 / 2;
    let y = (BACKGROUND_SIZE - dimensions.height * GRID_SIZE).clamp(0, BACKGROUND_SIZE) as u32 / 2;
    let index = layout.add_texture(URect::new(
        x,
        y,
        BACKGROUND_SIZE as u32 - x,
        BACKGROUND_SIZE as u32 - y,
    ));
    background_atlas.layout = texture_atlas_layouts.add(layout);
    background_atlas.index = index;

    transform_events.send(TransformEvent::Update);
}

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut transform_events: EventWriter<TransformEvent>,
) {
    if let Some(_event) = resize_reader.read().last() {
        transform_events.send(TransformEvent::Update);
    }
}

fn load_level(
    mut commands: Commands,
    mut background_query: Query<Entity, With<Background>>,
    mut dimensions: ResMut<Dimensions>,
    mut levels: ResMut<Levels>,
    mut pressed_triggers: ResMut<PressedTriggers>,
    assets: Res<GameObjectAssets>,
) {
    if !levels.is_changed() {
        return;
    }

    if cfg!(unix) {
        let current_level = levels.current_level;
        match fs::read_to_string(get_level_filename(current_level + 1)) {
            Ok(content) => levels.levels[current_level] = content.into(),
            Err(error) => println!("Could not read level: {error}"),
        }
    }

    let level = Level::load(&levels.levels[levels.current_level]);

    let background_entity = background_query
        .get_single_mut()
        .expect("there should be only one background");

    let mut background = commands.entity(background_entity);
    background.despawn_descendants();
    background.with_children(|cb| {
        spawn_level_objects(cb, level.objects, &assets);
    });

    pressed_triggers.num_pressed_triggers = 0;

    *dimensions = level.dimensions;
}

fn save_level(
    mut events: EventReader<SaveLevelEvent>,
    mut levels: ResMut<Levels>,
    dimensions: Res<Dimensions>,
    objects_query: Query<(&ObjectType, &Position, Option<&Direction>)>,
) {
    let Some(_event) = events.read().last() else {
        return;
    };

    let mut objects = BTreeMap::new();
    for (object_type, position, direction) in &objects_query {
        if position.x > 0
            && position.x <= dimensions.width
            && position.y > 0
            && position.y <= dimensions.height
        {
            let positions = objects.entry(*object_type).or_insert(Vec::new());
            positions.push(InitialPositionAndDirection {
                position: *position,
                direction: direction.copied(),
            });
        }
    }

    if !objects
        .get(&ObjectType::Player)
        .is_some_and(|player_locations| player_locations.len() == 1)
    {
        return; // Only save levels with exactly one player.
    }

    let level = Level {
        dimensions: *dimensions,
        objects,
    };
    let content = level.save();
    let current_level = levels.current_level;

    if cfg!(unix) {
        if let Err(error) = fs::write(get_level_filename(current_level + 1), &content) {
            println!("Could not save level: {error}");
        }
    }

    levels.levels[current_level] = content.into();
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

#[allow(clippy::too_many_arguments)]
fn toggle_editor(
    mut commands: Commands,
    mut events: EventReader<EditorEvent>,
    mut transform_events: EventWriter<TransformEvent>,
    mut movement_timer: ResMut<MovementTimer>,
    mut selected_object_type: ResMut<SelectedObjectType>,
    mut temporary_timer: ResMut<TemporaryTimer>,
    editor_query: Query<Entity, With<Editor>>,
    assets: Res<GameObjectAssets>,
    dimensions: Res<Dimensions>,
    fonts: Res<Fonts>,
) {
    let Some(_event) = events.read().last() else {
        return;
    };

    if let Ok(editor) = editor_query.get_single() {
        commands.entity(editor).despawn_recursive();
        **selected_object_type = None;

        movement_timer.unpause();
        temporary_timer.unpause();
    } else {
        commands
            .spawn(EditorBundle::new())
            .with_children(|cb| EditorBundle::populate(cb, &assets, &dimensions, &fonts));

        movement_timer.pause();
        temporary_timer.pause();
    }

    transform_events.send(TransformEvent::Update);
}

fn update_level_transform(
    mut events: EventReader<TransformEvent>,
    mut background_query: Query<&mut Transform, With<Background>>,
    player_query: Query<&Position, With<Player>>,
    editor_query: Query<Entity, With<Editor>>,
    dimensions: Res<Dimensions>,
    window_query: Query<&Window>,
    zoom: Res<Zoom>,
) {
    let Some(_event) = events.read().last() else {
        return;
    };

    let editor_open = editor_query.get_single().is_ok();
    let player_position = player_query
        .get_single()
        .expect("there should be only one player");
    let mut transform = background_query
        .get_single_mut()
        .expect("there should be only one background");
    let window = window_query
        .get_single()
        .expect("there should be only one window");
    let window_size = window.size();

    transform.scale = Vec3::new(zoom.factor, zoom.factor, 1.);

    let editor_width = if editor_open { EDITOR_WIDTH as f32 } else { 0. };
    let level_width = (dimensions.width * GRID_SIZE) as f32 * zoom.factor;
    let x = if level_width > window_size.x - editor_width {
        let max = 0.5 * (level_width - (window_size.x - editor_width));
        (zoom.factor
            * ((-player_position.x as f32 + 0.5 * dimensions.width as f32) + 0.5)
            * GRID_SIZE as f32)
            .clamp(-max, max)
    } else {
        0.
    };
    let level_height = (dimensions.height * GRID_SIZE) as f32 * zoom.factor;
    let y = if level_height > window_size.y {
        let max = 0.5 * (level_height - window_size.y);
        (zoom.factor
            * ((player_position.y as f32 - 0.5 * dimensions.height as f32) - 0.5)
            * GRID_SIZE as f32)
            .clamp(-max, max)
    } else {
        0.
    };
    transform.translation = Vec3::new(x - if editor_open { 0.5 * editor_width } else { 0. }, y, 1.);
}
