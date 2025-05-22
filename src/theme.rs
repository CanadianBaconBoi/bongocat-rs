use egui::Image;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ThemeSet {
    pub themes: Vec<AppTheme>,
}

impl ThemeSet {
    pub fn first(&self) -> &AppTheme {
        self.themes.first().unwrap()
    }
}

impl Default for ThemeSet {
    fn default() -> Self {
        ThemeSet {
            themes: Vec::from([AppTheme::default()]),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppTheme {
    pub paws_both: Image<'static>,
    pub paws_left: Image<'static>,
    pub paws_right: Image<'static>,
    pub paws_up: Image<'static>,
}

impl AppTheme {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        let path = path.into();
        let path_display = path.display();
        Self {
            paws_both: Image::from_uri(format!("file://{path_display}/paws_both.png")),
            paws_left: Image::from_uri(format!("file://{path_display}/paws_left.png")),
            paws_right: Image::from_uri(format!("file://{path_display}/paws_right.png")),
            paws_up: Image::from_uri(format!("file://{path_display}/paws_up.png")),
        }
    }
}

impl Default for AppTheme {
    fn default() -> Self {
        Self {
            paws_both: Image::new(egui::include_image!("../assets/frames/paws_both.png")),
            paws_left: Image::new(egui::include_image!("../assets/frames/paws_left.png")),
            paws_right: Image::new(egui::include_image!("../assets/frames/paws_right.png")),
            paws_up: Image::new(egui::include_image!("../assets/frames/paws_up.png")),
        }
    }
}
