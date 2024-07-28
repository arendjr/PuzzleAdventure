use std::str::FromStr;

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::errors::UnknownObjectType;

use super::{
    assets::GameObjectAssets,
    components::Position,
    object_bundles::{
        BlueBlockBundle, BouncingBallBundle, Creature1Bundle, ExitBundle, PlayerBundle, RaftBundle,
        RedBlockBundle, WaterBundle,
    },
    ButtonBundle, Direction, GateBundle, MineBundle,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ObjectType {
    BlueBlock,
    BouncingBall,
    Button,
    Creature1,
    Exit,
    Gate,
    Mine,
    Player,
    Raft,
    RedBlock,
    Water,
}

impl FromStr for ObjectType {
    type Err = UnknownObjectType;

    fn from_str(object_type: &str) -> Result<Self, Self::Err> {
        match object_type {
            "BlueBlock" => Ok(Self::BlueBlock),
            "BouncingBall" => Ok(Self::BouncingBall),
            "Button" => Ok(Self::Button),
            "Creature1" => Ok(Self::Creature1),
            "Exit" => Ok(Self::Exit),
            "Gate" => Ok(Self::Gate),
            "Mine" => Ok(Self::Mine),
            "Player" => Ok(Self::Player),
            "Raft" => Ok(Self::Raft),
            "RedBlock" => Ok(Self::RedBlock),
            "Water" => Ok(Self::Water),
            _ => Err(UnknownObjectType),
        }
    }
}

pub fn spawn_object_of_type<'a>(
    commands: &'a mut ChildBuilder,
    assets: &GameObjectAssets,
    object_type: ObjectType,
    position: Position,
    direction: Direction,
) -> EntityCommands<'a> {
    match object_type {
        ObjectType::BlueBlock => commands.spawn(BlueBlockBundle::spawn(assets, position)),
        ObjectType::BouncingBall => {
            commands.spawn(BouncingBallBundle::spawn(assets, position, direction))
        }
        ObjectType::Button => commands.spawn(ButtonBundle::spawn(assets, position)),
        ObjectType::Creature1 => {
            commands.spawn(Creature1Bundle::spawn(assets, position, direction))
        }
        ObjectType::Exit => commands.spawn(ExitBundle::spawn(assets, position)),
        ObjectType::Gate => commands.spawn(GateBundle::spawn(assets, position)),
        ObjectType::Mine => commands.spawn(MineBundle::spawn(assets, position)),
        ObjectType::Player => commands.spawn(PlayerBundle::spawn(assets, position)),
        ObjectType::Raft => commands.spawn(RaftBundle::spawn(assets, position)),
        ObjectType::RedBlock => commands.spawn(RedBlockBundle::spawn(assets, position)),
        ObjectType::Water => commands.spawn(WaterBundle::spawn(assets, position)),
    }
}
