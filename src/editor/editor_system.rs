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

#[allow(clippy::too_many_arguments)]
pub fn spawn_selected_object(
    mut commands: Commands,
    background_query: Query<(Entity, &Transform), With<Background>>,
    objects: Query<(Entity, &Position)>,
    selected_object_type: Res<SelectedObjectType>,
    buttons: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
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

    let x = (cursor_position.x - (window_size.x / 2. + transform.translation.x) * transform.scale.x)
        as i16
        / GRID_SIZE;
    let y = (cursor_position.y - (window_size.y / 2. + transform.translation.y) * transform.scale.y)
        as i16
        / GRID_SIZE;

    let position = Position { x, y };

    if let Some((object_type, direction)) = selected_object_type.get_object_type_and_direction() {
        let mut background = commands.entity(background);

        background.with_children(|cb| {
            spawn_object_of_type(cb, &assets, object_type, position, direction);
        });
    } else {
        for (entity, object_position) in &objects {
            if *object_position == position {
                commands.entity(entity).despawn();
            }
        }
    }
}
