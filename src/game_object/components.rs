use bevy::prelude::*;

#[derive(Component, Eq, PartialEq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

/// A deadly entity will kill the player if it comes into contact with it.
#[derive(Component)]
pub struct Deadly;

/// An exit completes the level when stepped on.
#[derive(Component)]
pub struct Exit;

/// A floatable entity will not sink when it comes into contact with a liquid.
#[derive(Component)]
pub struct Floatable;

/// Liquid entities will cause other entities to sinkn when it comes into
/// contact with them. An exception are [Floatable] entities.
#[derive(Component)]
pub struct Liquid;

/// Means the entity is controlled by the player.
#[derive(Component)]
pub struct Player;

/// A massive entity will prevent other entities from moving onto it.
///
/// An entity that is both massive and [Movable] will move first, but prevent
/// other entities from moving when it cannot be pushed further.
#[derive(Component)]
pub struct Massive;

/// A movable entity will be "pushed" if possible when another entity attempts
/// to move onto it.
#[derive(Component)]
pub struct Movable;
