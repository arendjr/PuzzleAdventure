use bevy::{
    color::palettes::{css::WHITE, tailwind::GRAY_950},
    prelude::*,
};

use crate::fonts::Fonts;

#[derive(Clone, Component, Copy)]
pub enum Button {
    Save,
}

#[derive(Bundle)]
pub struct EditorButtonBundle {
    button: ButtonBundle,
}

impl EditorButtonBundle {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(marker: impl Bundle) -> impl Bundle {
        (
            marker,
            Self {
                button: ButtonBundle {
                    background_color: GRAY_950.into(),
                    border_radius: BorderRadius::all(Val::Px(4.)),
                    style: Style {
                        height: Val::Px(30.),
                        width: Val::Px(100.),
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
        )
    }

    pub fn populate(cb: &mut ChildBuilder, text: impl Into<String>, fonts: &Fonts) {
        cb.spawn(TextBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font: fonts.poppins_light.clone(),
                    font_size: 18.,
                    color: WHITE.into(),
                },
            ),
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
