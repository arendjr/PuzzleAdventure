use bevy::{
    color::palettes::{
        css::WHITE,
        tailwind::{GRAY_600, GRAY_950},
    },
    prelude::*,
};

use crate::GameEvent;

use super::{number_input::NumberInput, Input};

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
