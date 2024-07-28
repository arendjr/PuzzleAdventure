use bevy::prelude::*;

use super::{
    assets::GameObjectAssets,
    components::{Exit, Liquid, Massive, Player, Position, Pushable},
    Animatable, Deadly, Direction, Floatable, Movable,
};

#[derive(Bundle)]
pub struct BlueBlockBundle {
    massive: Massive,
    position: Position,
    pushable: Pushable,
    sprite: SpriteBundle,
}

impl BlueBlockBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            massive: Massive,
            position,
            pushable: Pushable,
            sprite: SpriteBundle {
                texture: assets.blue_block.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct BouncingBallBundle {
    deadly: Deadly,
    direction: Direction,
    movable: Movable,
    position: Position,
    sprite: SpriteBundle,
}

impl BouncingBallBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position, direction: Direction) -> Self {
        Self {
            deadly: Deadly,
            direction,
            movable: Movable::Bounce,
            position,
            sprite: SpriteBundle {
                texture: assets.bouncing_ball.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 4.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct Creature1Bundle {
    atlas: TextureAtlas,
    deadly: Deadly,
    direction: Direction,
    movable: Movable,
    position: Position,
    sprite: SpriteBundle,
}

impl Creature1Bundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position, direction: Direction) -> Self {
        Self {
            atlas: TextureAtlas {
                layout: assets.creature1.1.clone(),
                index: direction as usize,
            },
            deadly: Deadly,
            direction,
            movable: Movable::FollowRightHand,
            position,
            sprite: SpriteBundle {
                texture: assets.creature1.0.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 4.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct ExitBundle {
    exit: Exit,
    position: Position,
    sprite: SpriteBundle,
}

impl ExitBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            exit: Exit,
            position,
            sprite: SpriteBundle {
                texture: assets.exit.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    position: Position,
    sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            player: Player,
            position,
            sprite: SpriteBundle {
                texture: assets.player.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 3.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct RaftBundle {
    floatable: Floatable,
    position: Position,
    pushable: Pushable,
    sprite: SpriteBundle,
}

impl RaftBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            floatable: Floatable,
            position,
            pushable: Pushable,
            sprite: SpriteBundle {
                texture: assets.raft.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct RedBlockBundle {
    massive: Massive,
    position: Position,
    sprite: SpriteBundle,
}

impl RedBlockBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            massive: Massive,
            position,
            sprite: SpriteBundle {
                texture: assets.red_block.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct WaterBundle {
    animatable: Animatable,
    atlas: TextureAtlas,
    liquid: Liquid,
    position: Position,
    sprite: SpriteBundle,
}

impl WaterBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            animatable: Animatable { num_frames: 3 },
            atlas: TextureAtlas {
                layout: assets.water.1.clone(),
                index: 0,
            },
            liquid: Liquid,
            position,
            sprite: SpriteBundle {
                texture: assets.water.0.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
        }
    }
}
