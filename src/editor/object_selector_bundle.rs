use std::ops::{Deref, DerefMut};

use bevy::{color::palettes::tailwind::GRAY_800, prelude::*};

use crate::{
    constants::{EDITOR_WIDTH, GRID_SIZE},
    game_object::{Direction, GameObjectAssets, ObjectType},
};

const NUM_OBJECTS: i16 = EditorObjectType::__Last as i16;
const NUM_COLUMNS: i16 = EDITOR_WIDTH / GRID_SIZE;
const NUM_ROWS: i16 =
    NUM_OBJECTS / NUM_COLUMNS + if NUM_OBJECTS % NUM_COLUMNS == 0 { 0 } else { 1 };
pub const SELECTOR_OUTLINE_WIDTH: i16 = 1;
const SELECTOR_WIDTH: i16 = NUM_COLUMNS * GRID_SIZE + (NUM_COLUMNS - 1) * SELECTOR_OUTLINE_WIDTH;
const SELECTOR_HEIGHT: i16 = NUM_ROWS * GRID_SIZE + (NUM_ROWS - 1) * SELECTOR_OUTLINE_WIDTH;

#[derive(Component)]
pub struct ObjectSelector;

#[derive(Default, Resource)]
pub struct SelectedObjectType(Option<EditorObjectType>);

impl Deref for SelectedObjectType {
    type Target = Option<EditorObjectType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SelectedObjectType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Component, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EditorObjectType {
    Eraser,
    BlueBlock,
    BouncingBallUp,
    BouncingBallRight,
    BouncingBallDown,
    BouncingBallLeft,
    Button,
    Creature1Up,
    Creature1Right,
    Creature1Down,
    Creature1Left,
    Exit,
    Gate,
    Mine,
    Player,
    Raft,
    RedBlock,
    Water,
    __Last,
}

impl EditorObjectType {
    pub fn get_object_type_and_direction(self) -> Option<(ObjectType, Direction)> {
        let object_type = match self {
            Self::BlueBlock => Some(ObjectType::BlueBlock),
            Self::BouncingBallUp
            | Self::BouncingBallRight
            | Self::BouncingBallDown
            | Self::BouncingBallLeft => Some(ObjectType::BouncingBall),
            Self::Button => Some(ObjectType::Button),
            Self::Creature1Up
            | Self::Creature1Right
            | Self::Creature1Down
            | Self::Creature1Left => Some(ObjectType::Creature1),
            Self::Exit => Some(ObjectType::Exit),
            Self::Gate => Some(ObjectType::Gate),
            Self::Mine => Some(ObjectType::Mine),
            Self::Player => Some(ObjectType::Player),
            Self::Raft => Some(ObjectType::Raft),
            Self::RedBlock => Some(ObjectType::RedBlock),
            Self::Water => Some(ObjectType::Water),
            Self::Eraser | Self::__Last => None,
        };

        let direction = match self {
            Self::BouncingBallUp => Direction::Up,
            Self::BouncingBallRight => Direction::Right,
            Self::BouncingBallDown => Direction::Down,
            Self::BouncingBallLeft => Direction::Left,
            Self::Creature1Up => Direction::Up,
            Self::Creature1Right => Direction::Right,
            Self::Creature1Down => Direction::Down,
            Self::Creature1Left => Direction::Left,
            _ => Direction::default(),
        };

        object_type.map(|object_type| (object_type, direction))
    }

    fn get_texture(self, assets: &GameObjectAssets) -> (Handle<Image>, Option<TextureAtlas>) {
        let image = match self {
            Self::Eraser => assets.eraser.clone(),
            Self::BlueBlock => assets.blue_block.clone(),
            Self::BouncingBallUp
            | Self::BouncingBallRight
            | Self::BouncingBallDown
            | Self::BouncingBallLeft => assets.bouncing_ball_editor.0.clone(),
            Self::Button => assets.button.clone(),
            Self::Creature1Up => assets.creature1.0.clone(),
            Self::Creature1Right => assets.creature1.0.clone(),
            Self::Creature1Down => assets.creature1.0.clone(),
            Self::Creature1Left => assets.creature1.0.clone(),
            Self::Exit => assets.exit.clone(),
            Self::Gate => assets.gate.0.clone(),
            Self::Mine => assets.mine.clone(),
            Self::Player => assets.player.clone(),
            Self::Raft => assets.raft.clone(),
            Self::RedBlock => assets.red_block.clone(),
            Self::Water => assets.water.0.clone(),
            Self::__Last => unreachable!(),
        };

        let atlas = match self {
            Self::BouncingBallUp => Some(TextureAtlas {
                layout: assets.bouncing_ball_editor.1.clone(),
                index: 0,
            }),
            Self::BouncingBallRight => Some(TextureAtlas {
                layout: assets.bouncing_ball_editor.1.clone(),
                index: 1,
            }),
            Self::BouncingBallDown => Some(TextureAtlas {
                layout: assets.bouncing_ball_editor.1.clone(),
                index: 2,
            }),
            Self::BouncingBallLeft => Some(TextureAtlas {
                layout: assets.bouncing_ball_editor.1.clone(),
                index: 3,
            }),
            Self::Creature1Up => Some(TextureAtlas {
                layout: assets.creature1.1.clone(),
                index: 0,
            }),
            Self::Creature1Right => Some(TextureAtlas {
                layout: assets.creature1.1.clone(),
                index: 1,
            }),
            Self::Creature1Down => Some(TextureAtlas {
                layout: assets.creature1.1.clone(),
                index: 2,
            }),
            Self::Creature1Left => Some(TextureAtlas {
                layout: assets.creature1.1.clone(),
                index: 3,
            }),
            Self::Gate => Some(TextureAtlas {
                layout: assets.gate.1.clone(),
                index: 0,
            }),
            Self::Water => Some(TextureAtlas {
                layout: assets.water.1.clone(),
                index: 0,
            }),
            _ => None,
        };

        (image, atlas)
    }
}

impl TryFrom<i16> for EditorObjectType {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let object_type = match value {
            0 => Self::Eraser,
            1 => Self::Player,
            2 => Self::Exit,
            3 => Self::RedBlock,
            4 => Self::BlueBlock,
            5 => Self::BouncingBallUp,
            6 => Self::BouncingBallRight,
            7 => Self::BouncingBallDown,
            8 => Self::BouncingBallLeft,
            9 => Self::Water,
            10 => Self::Creature1Up,
            11 => Self::Creature1Right,
            12 => Self::Creature1Down,
            13 => Self::Creature1Left,
            14 => Self::Raft,
            15 => Self::Gate,
            16 => Self::Button,
            17 => Self::Mine,
            _ => return Err(()),
        };
        Ok(object_type)
    }
}

#[derive(Bundle)]
pub struct ObjectSelectorBundle {
    background: NodeBundle,
    selector: ObjectSelector,
}

impl ObjectSelectorBundle {
    pub fn new() -> Self {
        Self {
            background: NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Px(SELECTOR_WIDTH as f32),
                    height: Val::Px(SELECTOR_HEIGHT as f32),
                    grid_template_columns: (0..NUM_COLUMNS)
                        .map(|_| GridTrack::px(GRID_SIZE as f32))
                        .collect(),
                    grid_template_rows: (0..NUM_ROWS)
                        .map(|_| GridTrack::px(GRID_SIZE as f32))
                        .collect(),
                    row_gap: Val::Px(SELECTOR_OUTLINE_WIDTH as f32),
                    column_gap: Val::Px(SELECTOR_OUTLINE_WIDTH as f32),
                    ..Default::default()
                },
                background_color: GRAY_800.into(),
                ..Default::default()
            },
            selector: ObjectSelector,
        }
    }

    pub fn populate(cb: &mut ChildBuilder, assets: &GameObjectAssets) {
        for i in 0..NUM_OBJECTS {
            let object_type = EditorObjectType::try_from(i).unwrap();
            let (texture, atlas) = object_type.get_texture(assets);
            let image = ImageBundle {
                image: UiImage::new(texture),
                ..Default::default()
            };
            let interaction = Interaction::None;

            if let Some(atlas) = atlas {
                cb.spawn((object_type, interaction, image, atlas));
            } else {
                cb.spawn((object_type, interaction, image));
            }
        }
    }
}
