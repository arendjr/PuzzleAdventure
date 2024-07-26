use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        texture::{CompressedImageFormats, ImageSampler, ImageType},
    },
};

pub fn load_asset(bytes: &[u8]) -> Image {
    Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::all(),
        true,
        ImageSampler::Default,
        RenderAssetUsages::all(),
    )
    .expect("cannot load game object asset")
}
