use bevy::{
    color::palettes::{
        css::WHITE,
        tailwind::{GRAY_600, GRAY_950},
    },
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    constants::GRID_SIZE,
    game_object::{spawn_object_of_type, GameObjectAssets, Position},
    level::Dimensions,
    Background, GameEvent,
};

use super::{number_input::NumberInput, Input, SelectedObjectType};

pub fn on_editor_number_input(
    mut interaction_query: Query<
        (&Interaction, &Input, &NumberInput, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut events: EventWriter<GameEvent>,
) {
    for (interaction, input, number_input, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = WHITE.into();

                let event = match (input, number_input) {
                    (Input::Width, NumberInput::Increase) => GameEvent::ChangeWidth(1),
                    (Input::Width, NumberInput::Decrease) => GameEvent::ChangeWidth(-1),
                    (Input::Height, NumberInput::Increase) => GameEvent::ChangeHeight(1),
                    (Input::Height, NumberInput::Decrease) => GameEvent::ChangeHeight(-1),
                    _ => continue,
                };
                events.send(event);
            }
            Interaction::Hovered => {
                *color = GRAY_600.into();
            }
            Interaction::None => {
                *color = GRAY_950.into();
            }
        }
    }
}

pub fn on_dimensions_changed(
    mut input_query: Query<(&Input, &NumberInput, &mut Text)>,
    dimensions: Res<Dimensions>,
) {
    if !dimensions.is_changed() {
        return;
    }

    for (input, number_input, mut text) in &mut input_query {
        match (input, number_input) {
            (Input::Width, NumberInput::Value) => {
                text.sections[0].value = dimensions.width.to_string()
            }
            (Input::Height, NumberInput::Value) => {
                text.sections[0].value = dimensions.height.to_string()
            }
            _ => continue,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_selected_object(
    mut commands: Commands,
    background_query: Query<(Entity, &Transform), With<Background>>,
    objects: Query<(Entity, &Position)>,
    selected_object_type: Res<SelectedObjectType>,
    buttons: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    dimensions: Res<Dimensions>,
    assets: Res<GameObjectAssets>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(selected_object_type) = **selected_object_type else {
        return;
    };

    let window = window_query
        .get_single()
        .expect("there should be only one window");
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let window_size = window.size();

    let (background, transform) = background_query
        .get_single()
        .expect("there should be only one background");

    let x = (((cursor_position.x - (0.5 * window_size.x + transform.translation.x))
        / GRID_SIZE as f32)
        / transform.scale.x
        + 0.5 * dimensions.width as f32) as i16
        + 1;
    let y = (((cursor_position.y - (0.5 * window_size.y - transform.translation.y))
        / GRID_SIZE as f32)
        / transform.scale.y
        + 0.5 * dimensions.height as f32) as i16
        + 1;

    let position = Position { x, y };

    for (entity, object_position) in &objects {
        if *object_position == position {
            commands.entity(entity).despawn();
        }
    }

    if x < 1 || x > dimensions.width || y < 1 || y > dimensions.height {
        return;
    }

    if let Some((object_type, direction)) = selected_object_type.get_object_type_and_direction() {
        let mut background = commands.entity(background);

        background.with_children(|cb| {
            spawn_object_of_type(cb, &assets, object_type, position, direction);
        });
    }
}
