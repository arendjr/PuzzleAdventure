use std::{error::Error, str::FromStr};

use bevy::{ecs::system::EntityCommands, prelude::*};

use super::{
    assets::GameObjectAssets,
    components::Position,
    object_bundles::{
        BlueBlockBundle, ExitBundle, PlayerBundle, RaftBundle, RedBlockBundle, WaterBundle,
    },
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ObjectType {
    BlueBlock,
    Exit,
    Player,
    Raft,
    RedBlock,
    Water,
}

#[derive(Debug)]
pub struct UnknownObjectType;

impl std::fmt::Display for UnknownObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unknown object type")
    }
}

impl Error for UnknownObjectType {}

impl FromStr for ObjectType {
    type Err = UnknownObjectType;

    fn from_str(object_type: &str) -> Result<Self, Self::Err> {
        match object_type {
            "BlueBlock" => Ok(Self::BlueBlock),
            "Exit" => Ok(Self::Exit),
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
) -> EntityCommands<'a> {
    match object_type {
        ObjectType::BlueBlock => commands.spawn(BlueBlockBundle::spawn(assets, position)),
        ObjectType::Exit => commands.spawn(ExitBundle::spawn(assets, position)),
        ObjectType::Player => commands.spawn(PlayerBundle::spawn(assets, position)),
        ObjectType::Raft => commands.spawn(RaftBundle::spawn(assets, position)),
        ObjectType::RedBlock => commands.spawn(RedBlockBundle::spawn(assets, position)),
        ObjectType::Water => commands.spawn(WaterBundle::spawn(assets, position)),
    }
}
