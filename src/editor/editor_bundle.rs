use bevy::{
    color::palettes::{css::RED, tailwind::GRAY_900},
    prelude::*,
};

use crate::{constants::EDITOR_WIDTH, fonts::Fonts};

use super::number_input::NumberInputBundle;

const BORDER_WIDTH: f32 = 2.;

#[derive(Component)]
pub struct Editor;

#[derive(Clone, Component, Copy)]
pub enum Input {
    Width,
    Height,
}

#[derive(Bundle)]
pub struct EditorBundle {
    background: NodeBundle,
    editor: Editor,
}

impl EditorBundle {
    pub fn new() -> Self {
        Self {
            background: NodeBundle {
                style: Style {
                    width: Val::Px(EDITOR_WIDTH as f32 - BORDER_WIDTH),
                    height: Val::Percent(100.),
                    border: UiRect::left(Val::Px(BORDER_WIDTH)),
                    padding: UiRect::all(Val::Px(20.)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    right: Val::Px(0.),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                background_color: GRAY_900.into(),
                border_color: RED.into(),
                z_index: ZIndex::Global(100),
                ..Default::default()
            },
            editor: Editor,
        }
    }

    pub fn populate(cb: &mut ChildBuilder, fonts: &Fonts) {
        cb.spawn(NumberInputBundle::new())
            .with_children(|cb| NumberInputBundle::populate(cb, Input::Width, "Width", fonts));

        cb.spawn(NumberInputBundle::new())
            .with_children(|cb| NumberInputBundle::populate(cb, Input::Height, "Height", fonts));
    }
}
