use bevy::{
    color::palettes::{
        css::{RED, WHITE},
        tailwind::{GRAY_600, GRAY_800},
    },
    prelude::*,
};

use super::{EditorObjectType, SelectedObjectType, SELECTOR_OUTLINE_WIDTH};

pub fn on_object_selector_input(
    mut interaction_query: Query<
        (&Interaction, &EditorObjectType, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut selected_object_type: ResMut<SelectedObjectType>,
) {
    for (interaction, object_type, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = WHITE.into();

                **selected_object_type = Some(*object_type);
            }
            Interaction::Hovered => {
                *color = GRAY_600.into();
            }
            Interaction::None => {
                *color = GRAY_800.into();
            }
        }
    }
}

pub fn on_selected_object_change(
    mut commands: Commands,
    mut query: Query<(Entity, &EditorObjectType, Option<&mut Outline>)>,
    selected_object_type: Res<SelectedObjectType>,
) {
    if !selected_object_type.is_changed() {
        return;
    }

    for (entity, object_type, outline) in &mut query {
        let is_selected_object_type = selected_object_type.is_some_and(|ty| ty == *object_type);

        if let Some(mut outline) = outline {
            outline.color = if is_selected_object_type {
                RED.into()
            } else {
                Color::NONE
            };
        } else if is_selected_object_type {
            commands.entity(entity).insert(Outline::new(
                Val::Px(SELECTOR_OUTLINE_WIDTH as f32),
                Val::ZERO,
                RED.into(),
            ));
        }
    }
}
