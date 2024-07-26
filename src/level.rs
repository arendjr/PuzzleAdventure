use std::{
    collections::{btree_map::Entry, BTreeMap},
    str::FromStr,
};

use bevy::prelude::Resource;

use crate::game_object::{ObjectType, Position};

pub const LEVELS: &[&str] = &[
    include_str!("../assets/levels/level001"),
    include_str!("../assets/levels/level002"),
    include_str!("../assets/levels/level003"),
    include_str!("../assets/levels/level004"),
    include_str!("../assets/levels/level005"),
    include_str!("../assets/levels/level006"),
    include_str!("../assets/levels/level007"),
    include_str!("../assets/levels/level008"),
    include_str!("../assets/levels/level009"),
    include_str!("../assets/levels/level010"),
    include_str!("../assets/levels/level011"),
    include_str!("../assets/levels/level012"),
    include_str!("../assets/levels/level013"),
    include_str!("../assets/levels/level014"),
    include_str!("../assets/levels/level015"),
];

#[derive(Resource)]
pub struct Dimensions {
    pub width: i16,
    pub height: i16,
}

impl Default for Dimensions {
    fn default() -> Self {
        Self {
            width: 16,
            height: 16,
        }
    }
}

pub struct Level {
    pub dimensions: Dimensions,
    pub objects: BTreeMap<ObjectType, Vec<Position>>,
}

pub fn load_level(content: &str) -> Level {
    let mut dimensions = Dimensions::default();
    let mut objects: BTreeMap<ObjectType, Vec<Position>> = BTreeMap::new();

    let mut section_name = None;
    for line in content.lines() {
        let line = line.trim();

        if line.starts_with('[') && line.ends_with(']') {
            section_name = Some(&line[1..line.len() - 1]);
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        let Some(section_name) = section_name else {
            continue;
        };

        if section_name == "General" {
            match (key, value.parse()) {
                ("Width", Ok(value)) => dimensions.width = value,
                ("Height", Ok(value)) => dimensions.height = value,
                (_, Ok(_)) => println!("Unknown key: {key}"),
                (_, Err(error)) => println!("Invalid dimension in key {key}: {error}"),
            }
            continue;
        }

        let object_type = match ObjectType::from_str(section_name) {
            Ok(object_type) => object_type,
            Err(_) => {
                println!("Unknown object type: {section_name}");
                continue;
            }
        };

        if key == "Position" {
            let positions: Vec<Position> = value
                .split(';')
                .filter_map(|location| match location.split_once(',') {
                    Some((x, y)) => match (x.parse(), y.parse()) {
                        (Ok(x), Ok(y)) => Some(Position { x, y }),
                        _ => {
                            println!("Invalid location ({x},{y})");
                            None
                        }
                    },
                    _ => None,
                })
                .collect();

            if !positions.is_empty() {
                let entry = objects.entry(object_type);
                match entry {
                    Entry::Occupied(mut entry) => entry.get_mut().extend(positions),
                    Entry::Vacant(entry) => {
                        entry.insert(positions);
                    }
                }
            }
        } else {
            println!("Unknown key: {key}");
        }
    }

    if !objects.contains_key(&ObjectType::Player) {
        println!("Warning: Level didn't contain a player");
    }

    Level {
        dimensions,
        objects,
    }
}
