use egui::{ColorImage, Context, TextureHandle, TextureOptions};
use image::DynamicImage;
use std::path::Path;

pub fn load_texture_from_color_image(
    context: &Context,
    image: &ColorImage,
    name: String,
) -> TextureHandle {
    context.load_texture(
        name,
        image.clone(),
        TextureOptions::default().with_mipmap_mode(None),
    )
}

pub fn load_color_image_from_path(image: &impl AsRef<Path>) -> ColorImage {
    let image = image::ImageReader::open(image).unwrap().decode().unwrap();
    color_image_from_dynamic(image)
}

pub fn color_image_from_dynamic(image: DynamicImage) -> ColorImage {
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
}
