use bevy::prelude::*;

use crate::utils::load_asset;

#[derive(Clone, Default, Resource)]
pub struct GameObjectAssets {
    pub blue_block: Handle<Image>,
    pub bouncing_ball: Handle<Image>,
    pub creature1: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub exit: Handle<Image>,
    pub player: Handle<Image>,
    pub raft: Handle<Image>,
    pub red_block: Handle<Image>,
    pub water: (Handle<Image>, Handle<TextureAtlasLayout>),
}

impl GameObjectAssets {
    pub fn load(
        images: &mut ResMut<Assets<Image>>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let one_by_three_atlas = {
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 1, 3, None, None);
            texture_atlas_layouts.add(layout)
        };
        let one_by_four_atlas = {
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 1, 4, None, None);
            texture_atlas_layouts.add(layout)
        };

        Self {
            blue_block: images.add(load_asset(include_bytes!(
                "../../assets/sprites/blueblock.png"
            ))),
            bouncing_ball: images.add(load_asset(include_bytes!(
                "../../assets/sprites/greenball.png"
            ))),
            creature1: (
                images.add(load_asset(include_bytes!(
                    "../../assets/sprites/creature1.png"
                ))),
                one_by_four_atlas,
            ),
            exit: images.add(load_asset(include_bytes!("../../assets/sprites/exit.png"))),
            player: images.add(load_asset(include_bytes!(
                "../../assets/sprites/player.png"
            ))),
            raft: images.add(load_asset(include_bytes!("../../assets/sprites/raft.png"))),
            red_block: images.add(load_asset(include_bytes!(
                "../../assets/sprites/redblock.png"
            ))),
            water: (
                images.add(load_asset(include_bytes!("../../assets/sprites/water.png"))),
                one_by_three_atlas,
            ),
        }
    }
}
