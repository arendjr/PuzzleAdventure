use bevy::prelude::*;

use crate::utils::load_asset;

#[derive(Clone, Default, Resource)]
pub struct GameObjectAssets {
    pub blue_block: Handle<Image>,
    pub exit: Handle<Image>,
    pub player: Handle<Image>,
    pub red_block: Handle<Image>,
    pub water: Handle<Image>,
}

impl GameObjectAssets {
    pub fn load(images: &mut ResMut<Assets<Image>>) -> Self {
        Self {
            blue_block: images.add(load_asset(include_bytes!(
                "../../assets/sprites/blueblock.png"
            ))),
            exit: images.add(load_asset(include_bytes!("../../assets/sprites/exit.png"))),
            player: images.add(load_asset(include_bytes!(
                "../../assets/sprites/player.png"
            ))),
            red_block: images.add(load_asset(include_bytes!(
                "../../assets/sprites/redblock.png"
            ))),
            water: images.add(load_asset(include_bytes!("../../assets/sprites/water.png"))),
        }
    }
}
