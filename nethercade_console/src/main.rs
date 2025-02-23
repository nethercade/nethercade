mod console_app;
use console_app::*;

mod audio;
mod console;
mod graphics;

use eframe::egui;
use eframe::wgpu;
use nethercade_core::Resolution;

pub const MAX_PUSH_CONSTANT_SIZE: u32 = 128;

fn main() {
    println!("Launching Console...");

    let dimensions = Resolution::Compact.dimensions();

    let device_descriptor = std::sync::Arc::new(|_: &wgpu::Adapter| {
        let limits = wgpu::Limits {
            max_push_constant_size: MAX_PUSH_CONSTANT_SIZE,
            ..Default::default()
        };
        wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::PUSH_CONSTANTS,
            required_limits: limits,
            memory_hints: wgpu::MemoryHints::default(),
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([dimensions.0 as f32, dimensions.1 as f32]),
        centered: true,
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
            wgpu_setup: eframe::egui_wgpu::WgpuSetup::CreateNew {
                supported_backends: eframe::wgpu::Backends::default(),
                power_preference: eframe::wgpu::PowerPreference::HighPerformance,
                device_descriptor,
            },
            ..Default::default()
        },
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
