use std::fmt::Display;
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
    ButtonBundle, Direction, GateBundle, MineBundle, TransporterBundle,
};

#[derive(Clone, Component, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
    Transporter,
    Water,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ObjectType::BlueBlock => "BlueBlock",
            ObjectType::BouncingBall => "BouncingBall",
            ObjectType::Button => "Button",
            ObjectType::Creature1 => "Creature1",
            ObjectType::Exit => "Exit",
            ObjectType::Gate => "Gate",
            ObjectType::Mine => "Mine",
            ObjectType::Player => "Player",
            ObjectType::Raft => "Raft",
            ObjectType::RedBlock => "RedBlock",
            ObjectType::Transporter => "Transporter",
            ObjectType::Water => "Water",
        })
    }
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
            "Transporter" => Ok(Self::Transporter),
            "Water" => Ok(Self::Water),
            _ => Err(UnknownObjectType),
        }
    }
}

pub fn spawn_object_of_type<'a>(
    cb: &'a mut ChildBuilder,
    assets: &GameObjectAssets,
    object_type: ObjectType,
    position: Position,
    direction: Direction,
) -> EntityCommands<'a> {
    match object_type {
        ObjectType::BlueBlock => cb.spawn(BlueBlockBundle::spawn(assets, position)),
        ObjectType::BouncingBall => {
            cb.spawn(BouncingBallBundle::spawn(assets, position, direction))
        }
        ObjectType::Button => cb.spawn(ButtonBundle::spawn(assets, position)),
        ObjectType::Creature1 => cb.spawn(Creature1Bundle::spawn(assets, position, direction)),
        ObjectType::Exit => cb.spawn(ExitBundle::spawn(assets, position)),
        ObjectType::Gate => cb.spawn(GateBundle::spawn(assets, position)),
        ObjectType::Mine => cb.spawn(MineBundle::spawn(assets, position)),
        ObjectType::Player => cb.spawn(PlayerBundle::spawn(assets, position)),
        ObjectType::Raft => cb.spawn(RaftBundle::spawn(assets, position)),
        ObjectType::RedBlock => cb.spawn(RedBlockBundle::spawn(assets, position)),
        ObjectType::Transporter => cb.spawn(TransporterBundle::spawn(assets, position, direction)),
        ObjectType::Water => cb.spawn(WaterBundle::spawn(assets, position)),
    }
}
