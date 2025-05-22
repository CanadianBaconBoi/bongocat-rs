use crate::app::color_image_from_dynamic;
use egui::{ColorImage, TextureHandle};
use std::path::PathBuf;

#[derive(Clone)]
pub struct ThemeSet {
    pub themes: Vec<AppTheme>,
    pub themes_loaded: Vec<AppThemeImage>,
    pub themes_rendered: Vec<AppThemeTexture>,
}

impl Default for ThemeSet {
    fn default() -> Self {
        ThemeSet {
            themes: vec![
            ],
            themes_loaded: vec![AppThemeImage {
                app_theme: None,
                paws_both: color_image_from_dynamic(
                    image::load_from_memory(include_bytes!("../assets/frames/paws_both.png")).unwrap(),
                )
                    .unwrap(),
                paws_left: color_image_from_dynamic(
                    image::load_from_memory(include_bytes!("../assets/frames/paws_left.png")).unwrap(),
                )
                    .unwrap(),
                paws_right: color_image_from_dynamic(
                    image::load_from_memory(include_bytes!("../assets/frames/paws_right.png")).unwrap(),
                )
                    .unwrap(),
                paws_up: color_image_from_dynamic(
                    image::load_from_memory(include_bytes!("../assets/frames/paws_up.png")).unwrap(),
                )
                    .unwrap(),
            },
            AppThemeImage {
                app_theme: None,
                paws_both: color_image_from_dynamic(
                    image::load_from_memory(include_bytes!("../assets/frames/o/paws_both.png")).unwrap(),
                )
                    .unwrap(),
                paws_left: color_image_from_dynamic(
                    image::load_from_memory(include_bytes!("../assets/frames/o/paws_left.png")).unwrap(),
                )
                    .unwrap(),
                paws_right: color_image_from_dynamic(
                    image::load_from_memory(include_bytes!("../assets/frames/o/paws_right.png")).unwrap(),
                )
                    .unwrap(),
                paws_up: color_image_from_dynamic(
                    image::load_from_memory(include_bytes!("../assets/frames/o/paws_up.png")).unwrap(),
                )
                    .unwrap(),
            }],
            themes_rendered: vec![],
        }
    }
}

#[derive(Clone)]
pub struct AppThemeTexture {
    pub app_theme_image: AppThemeImage,
    pub paws_both: Option<TextureHandle>,
    pub paws_left: Option<TextureHandle>,
    pub paws_right: Option<TextureHandle>,
    pub paws_up: Option<TextureHandle>,
}

#[derive(Clone, Debug)]
pub struct AppThemeImage {
    pub app_theme: Option<AppTheme>,
    pub paws_both: ColorImage,
    pub paws_left: ColorImage,
    pub paws_right: ColorImage,
    pub paws_up: ColorImage,
}

#[derive(Clone, Debug)]
pub struct AppTheme {
    pub paws_both: PathBuf,
    pub paws_left: PathBuf,
    pub paws_right: PathBuf,
    pub paws_up: PathBuf,
}

impl AppTheme {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        let path = path.into();
        let path_display = path.display();

        Self {
            paws_both: format!("{path_display}/paws_both.png").into(),
            paws_left: format!("{path_display}/paws_left.png").into(),
            paws_right: format!("{path_display}/paws_right.png").into(),
            paws_up: format!("{path_display}/paws_up.png").into(),
        }
    }
}