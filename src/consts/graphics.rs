pub const WINDOW_WIDTH: f32 = 250.0;
pub const WINDOW_HEIGHT: f32 = WINDOW_WIDTH;
pub const PADDING_PIXELS: u64 = 10;
pub const WINDOW_RECT: egui::Rect = egui::Rect::from_min_max(
    egui::Pos2::ZERO,
    egui::Pos2::new(WINDOW_WIDTH, WINDOW_HEIGHT),
);
pub const UV_RECT: egui::Rect =
    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0));
