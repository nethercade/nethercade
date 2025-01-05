mod console_app;
use console_app::*;

mod graphics;

use eframe::egui;

fn main() {
    println!("Launching Console...");

    // TODO: Parse the target game to pass into resolution
    // TODO: Make this refresh instead of be event driven

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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
