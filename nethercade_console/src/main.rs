mod console_app;
use console_app::*;

mod audio;
mod graphics;

use eframe::egui;
use nethercade_core::Resolution;

fn main() {
    println!("Launching Console...");

    // TODO: Parse the target game to pass into resolution
    // TODO: Make this refresh instead of be event driven

    let dimensions = Resolution::Compact.dimensions();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([dimensions.0 as f32, dimensions.1 as f32]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Nethercade Z",
        options,
        Box::new(|cc| {
            let app = ConsoleApp::new(cc).unwrap();
            Ok(Box::new(app))
        }),
    )
    .unwrap();

    println!("Console closing.");
}
