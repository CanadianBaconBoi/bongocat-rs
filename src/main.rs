#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![feature(stmt_expr_attributes)]

use bongocat_rs::app::BongoApp;
use bongocat_rs::consts::graphics::{WINDOW_HEIGHT, WINDOW_WIDTH};
use display_info::DisplayInfo;
use egui::WindowLevel;

fn main() -> eframe::Result {
    env_logger::init();

    let displays = DisplayInfo::all().unwrap();
    let primary = displays.iter().find(|d| d.is_primary).unwrap_or_else(|| {
        displays
            .first()
            .unwrap_or_else(|| panic!("Could not find primary display"))
    });

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_max_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_always_on_top()
            .with_mouse_passthrough(true)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_resizable(false)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            )
            .with_position([
                (primary.x + (primary.height as i32)) as f32 - WINDOW_HEIGHT,
                (primary.y + (primary.width as i32 / 2)) as f32 - WINDOW_WIDTH / 2.0,
            ])
            .with_drag_and_drop(false),
        ..Default::default()
    };

    eframe::run_native(
        "Bongocat",
        native_options,
        Box::new(|cc| Ok(Box::new(BongoApp::new(cc)))),
    )
}
