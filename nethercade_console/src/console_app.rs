use std::{ffi::OsStr, io::Read, sync::Arc};

use eframe::{
    egui::{self, Sense},
    egui_wgpu, wgpu,
};
use nethercade_core::Rom;

use crate::{console::Console, graphics::VirtualGpuCallback};

pub struct ConsoleApp {
    console: Option<Console>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    format: wgpu::TextureFormat,
}

impl ConsoleApp {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        // TODO: Preload whatever stuff needed
        let wgpu_render_state = cc.wgpu_render_state.as_ref()?;
        let queue = wgpu_render_state.queue.clone();

        let device = wgpu_render_state.device.clone();
        let format = wgpu_render_state.target_format;

        Some(Self {
            console: None,
            queue,
            device,
            format,
        })
    }
}

impl eframe::App for ConsoleApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // TODO: Render a File Menu
        egui::CentralPanel::default().show(ctx, |ui| match &mut self.console {
            Some(console) => {
                console.update();
                console.draw();

                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let (width, height) = console.rom.resolution.dimensions();
                    let (rect, _response) = ui.allocate_exact_size(
                        egui::Vec2::new(width as f32, height as f32),
                        Sense::click(),
                    );

                    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                        rect,
                        VirtualGpuCallback {
                            frame_buffer: console.get_frame_buffer(),
                        },
                    ));
                });
            }
            None => {
                if ui.button("Load Rom").clicked() {
                    if let Some(rom) = try_load_rom() {
                        self.console =
                            Some(Console::new(rom, &self.device, &self.queue, self.format));
                    }
                }
            }
        });

        // Render continiously
        ctx.request_repaint();
    }
}

fn try_load_rom() -> Option<Rom> {
    // TODO: Add filters for .nzrom and .wasm
    let path = rfd::FileDialog::new().pick_file()?;

    match path.extension().and_then(OsStr::to_str) {
        Some("nzrom") => {
            let mut file = std::fs::File::open(path).ok()?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).ok()?;
            Some(bitcode::decode(&bytes).ok()?)
        }
        Some("wasm") => {
            let mut file = std::fs::File::open(path).ok()?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).ok()?;
            Some(Rom::from_code(&bytes))
        }
        _ => panic!("Invalid File"),
    }
}
