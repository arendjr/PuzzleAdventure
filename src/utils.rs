use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        texture::{
            CompressedImageFormats, ImageAddressMode, ImageSampler, ImageSamplerDescriptor,
            ImageType,
        },
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

pub fn load_repeating_asset(bytes: &[u8]) -> Image {
    Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::all(),
        true,
        ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            ..default()
        }),
        RenderAssetUsages::all(),
    )
    .expect("cannot load game object asset")
}
