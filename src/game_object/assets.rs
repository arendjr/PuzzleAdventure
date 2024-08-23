use bevy::prelude::*;

use crate::utils::load_asset;

#[derive(Clone, Default, Resource)]
pub struct GameObjectAssets {
    pub blue_block: Handle<Image>,
    pub bouncing_ball: Handle<Image>,
    pub button: Handle<Image>,
    pub creature1: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub eraser: Handle<Image>,
    pub explosion: Handle<Image>,
    pub exit: Handle<Image>,
    pub gate: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub mine: Handle<Image>,
    pub player: Handle<Image>,
    pub raft: Handle<Image>,
    pub red_block: Handle<Image>,
    pub splash: Handle<Image>,
    pub water: (Handle<Image>, Handle<TextureAtlasLayout>),
}

impl GameObjectAssets {
    pub fn load(
        images: &mut ResMut<Assets<Image>>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let one_by_two_atlas = {
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 1, 2, None, None);
            texture_atlas_layouts.add(layout)
        };
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
            button: images.add(load_asset(include_bytes!(
                "../../assets/sprites/button.png"
            ))),
            creature1: (
                images.add(load_asset(include_bytes!(
                    "../../assets/sprites/creature1.png"
                ))),
                one_by_four_atlas,
            ),
            eraser: images.add(load_asset(include_bytes!(
                "../../assets/sprites/eraser.png"
            ))),
            exit: images.add(load_asset(include_bytes!("../../assets/sprites/exit.png"))),
            explosion: images.add(load_asset(include_bytes!(
                "../../assets/sprites/explosion.png"
            ))),
            gate: (
                images.add(load_asset(include_bytes!("../../assets/sprites/gate.png"))),
                one_by_two_atlas,
            ),
            mine: images.add(load_asset(include_bytes!("../../assets/sprites/mine.png"))),
            player: images.add(load_asset(include_bytes!(
                "../../assets/sprites/player.png"
            ))),
            raft: images.add(load_asset(include_bytes!("../../assets/sprites/raft.png"))),
            red_block: images.add(load_asset(include_bytes!(
                "../../assets/sprites/redblock.png"
            ))),
            splash: images.add(load_asset(include_bytes!(
                "../../assets/sprites/splash.png"
            ))),
            water: (
                images.add(load_asset(include_bytes!("../../assets/sprites/water.png"))),
                one_by_three_atlas,
            ),
        }
    }
}
