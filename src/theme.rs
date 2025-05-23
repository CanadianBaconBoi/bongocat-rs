use crate::app::helpers::color_image_from_dynamic;
use egui::{ColorImage, TextureHandle};
use image::ImageFormat;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct ThemeSet {
    pub themes: Vec<AppTheme>,
    pub themes_loaded: Vec<AppThemeImage>,
    pub themes_rendered: Vec<AppThemeTexture>,
}

impl Default for ThemeSet {
    fn default() -> Self {
        ThemeSet {
            themes: vec![],
            themes_loaded: vec![
                AppThemeImage {
                    id: "standard".to_string().into(),
                    paws_both: color_image_from_dynamic(
                        image::load_from_memory_with_format(
                            include_bytes!("../assets/frames/paws_both.png"),
                            ImageFormat::Png,
                        )
                        .unwrap(),
                    ),
                    paws_left: color_image_from_dynamic(
                        image::load_from_memory_with_format(
                            include_bytes!("../assets/frames/paws_left.png"),
                            ImageFormat::Png,
                        )
                        .unwrap(),
                    ),
                    paws_right: color_image_from_dynamic(
                        image::load_from_memory_with_format(
                            include_bytes!("../assets/frames/paws_right.png"),
                            ImageFormat::Png,
                        )
                        .unwrap(),
                    ),
                    paws_up: color_image_from_dynamic(
                        image::load_from_memory_with_format(
                            include_bytes!("../assets/frames/paws_up.png"),
                            ImageFormat::Png,
                        )
                        .unwrap(),
                    ),
                },
                AppThemeImage {
                    id: "standard-o".to_string().into(),
                    paws_both: color_image_from_dynamic(
                        image::load_from_memory_with_format(
                            include_bytes!("../assets/frames/o/paws_both.png"),
                            ImageFormat::Png,
                        )
                        .unwrap(),
                    ),
                    paws_left: color_image_from_dynamic(
                        image::load_from_memory_with_format(
                            include_bytes!("../assets/frames/o/paws_left.png"),
                            ImageFormat::Png,
                        )
                        .unwrap(),
                    ),
                    paws_right: color_image_from_dynamic(
                        image::load_from_memory_with_format(
                            include_bytes!("../assets/frames/o/paws_right.png"),
                            ImageFormat::Png,
                        )
                        .unwrap(),
                    ),
                    paws_up: color_image_from_dynamic(
                        image::load_from_memory_with_format(
                            include_bytes!("../assets/frames/o/paws_up.png"),
                            ImageFormat::Png,
                        )
                        .unwrap(),
                    ),
                },
            ],
            themes_rendered: vec![],
        }
    }
}

pub struct AppThemeTexture {
    pub id: Arc<String>,
    pub paws_both: Option<TextureHandle>,
    pub paws_left: Option<TextureHandle>,
    pub paws_right: Option<TextureHandle>,
    pub paws_up: Option<TextureHandle>,
}

pub struct AppThemeImage {
    pub id: Arc<String>,
    pub paws_both: ColorImage,
    pub paws_left: ColorImage,
    pub paws_right: ColorImage,
    pub paws_up: ColorImage,
}

pub struct AppTheme {
    pub id: Arc<String>,
    pub paws_both: PathBuf,
    pub paws_left: PathBuf,
    pub paws_right: PathBuf,
    pub paws_up: PathBuf,
}

impl AppTheme {
    pub fn new<P: AsRef<Path>>(id: String, path: P) -> Self {
        let path = path.as_ref();
        let path_display = path.display();

        Self {
            id: id.into(),
            paws_both: format!("{path_display}/paws_both.png").into(),
            paws_left: format!("{path_display}/paws_left.png").into(),
            paws_right: format!("{path_display}/paws_right.png").into(),
            paws_up: format!("{path_display}/paws_up.png").into(),
        }
    }
}
