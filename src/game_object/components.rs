use std::str::FromStr;

use bevy::prelude::*;

use crate::errors::UnknownDirection;

#[derive(Clone, Component, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Component, Copy, Debug, Default, Eq, PartialEq)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn inverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    pub fn left_hand(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    pub fn right_hand(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    /// Returns a `(dx, dy)` tuple for the direction.
    pub fn to_delta(self) -> (i16, i16) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = UnknownDirection;

    fn from_str(direction: &str) -> Result<Self, Self::Err> {
        match direction {
            "Up" => Ok(Self::Up),
            "Right" => Ok(Self::Right),
            "Down" => Ok(Self::Down),
            "Left" => Ok(Self::Left),
            _ => Err(UnknownDirection),
        }
    }
}

#[derive(Component)]
pub struct Animatable {
    pub num_frames: usize,
}

/// A deadly entity will kill the player if it comes into contact with it.
#[derive(Component)]
pub struct Deadly;

/// An exit completes the level when stepped on.
#[derive(Component)]
pub struct Exit;

/// Explodes on contact.
///
/// Should not be combined with [Deadly]. Dying is implied if the player
/// explodes.
#[derive(Component)]
pub struct Explosive;

/// A floatable entity will not sink when it comes into contact with a liquid.
#[derive(Component)]
pub struct Floatable;

/// Liquid entities will cause other entities to sink when it comes into
/// contact with them. An exception are [Floatable] entities.
///
/// Should not be combined with [Deadly]. Dying is implied if the player
/// sinks.
#[derive(Component)]
pub struct Liquid;

/// A massive entity will prevent other entities from moving onto it.
///
/// An entity that is both massive and [Movable] will move first, but prevent
/// other entities from moving when it cannot be pushed further.
#[derive(Component)]
pub struct Massive;

/// Movable entities move by themselves.
///
/// They face a given [Direction], while the [Movable] variant decides what will
/// be their next direction.
#[derive(Component)]
pub enum Movable {
    /// Bounces back in the opposite direction whenever they cannot move further
    /// in their current direction.
    Bounce,

    /// Turns right whenever they can, while following whatever obstacles they
    /// have on their right.
    FollowRightHand,
}

/// A [Massive] entity that can be opened by pressing a [Trigger].
#[derive(Component)]
pub struct Openable;

/// Entity is controlled by the player.
#[derive(Component)]
pub struct Player;

/// A movable entity will be "pushed" if possible when another entity attempts
/// to move onto it.
#[derive(Component)]
pub struct Pushable;

/// Entity acts as trigger for opening gates.
#[derive(Component)]
pub struct Trigger;

/// Automatically disappears after spawning.
#[derive(Component)]
pub struct Volatile;
