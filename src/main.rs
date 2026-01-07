mod app;
mod core;
mod render;
mod time;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Tetris Modern"),
        ..Default::default()
    };

    eframe::run_native(
        "Tetris Modern",
        options,
        Box::new(|cc| Ok(Box::new(app::TetrisApp::new(cc)))),
    )
}
