use bevy::{
    color::palettes::{css::WHITE, tailwind::GRAY_950},
    prelude::*,
};

use crate::fonts::Fonts;

#[derive(Component)]
pub enum NumberInput {
    Increase,
    Decrease,
}

#[derive(Bundle)]
pub struct NumberInputBundle {
    node: NodeBundle,
}

impl NumberInputBundle {
    pub fn new() -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Px(30.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    pub fn populate(
        cb: &mut ChildBuilder,
        marker: impl Component + Copy,
        text: &str,
        fonts: &Fonts,
    ) {
        cb.spawn(TextBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font: fonts.poppins_light.clone(),
                    font_size: 18.,
                    color: WHITE.into(),
                },
            ),
            ..Default::default()
        });

        cb.spawn(NodeBundle {
            style: Style {
                width: Val::Px(20.),
                height: Val::Px(22.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|cb| {
            cb.spawn((
                marker,
                Interaction::None,
                NumberInput::Increase,
                TextBundle {
                    style: Style {
                        width: Val::Px(20.),
                        height: Val::Px(10.),
                        margin: UiRect::bottom(Val::Px(1.)),
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "    +",
                        TextStyle {
                            font: fonts.poppins_light.clone(),
                            font_size: 10.,
                            color: WHITE.into(),
                        },
                    ),
                    background_color: GRAY_950.into(),
                    ..Default::default()
                },
            ));
            cb.spawn((
                marker,
                Interaction::None,
                NumberInput::Decrease,
                TextBundle {
                    style: Style {
                        width: Val::Px(20.),
                        height: Val::Px(10.),
                        margin: UiRect::top(Val::Px(1.)),
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "    -",
                        TextStyle {
                            font: fonts.poppins_light.clone(),
                            font_size: 10.,
                            color: WHITE.into(),
                        },
                    ),
                    background_color: GRAY_950.into(),
                    ..Default::default()
                },
            ));
        });
    }
}
