use std::{ffi::OsStr, io::Read};

use eframe::{
    egui::{self, Sense},
    egui_wgpu,
};
use nethercade_core::Rom;

use crate::{
    console::Console,
    graphics::{VirtualGpuCallback, VirtualGpuResources},
};

#[derive(Default)]
pub struct ConsoleApp {
    console: Option<Console>,
}

impl ConsoleApp {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        // TODO: Preload whatever stuff needed
        let wgpu_render_state = cc.wgpu_render_state.as_ref()?;

        let device = &wgpu_render_state.device;
        let format = wgpu_render_state.target_format;

        let vgpu = VirtualGpuResources::new(device, format);

        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(vgpu);

        Some(Self::default())
    }
}

impl eframe::App for ConsoleApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // TODO: Render a File Menu
        egui::CentralPanel::default().show(ctx, |ui| match &self.console {
            Some(_) => {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let (rect, _response) =
                        ui.allocate_exact_size(egui::Vec2::splat(300.0), Sense::click());

                    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                        rect,
                        VirtualGpuCallback,
                    ));
                });
            }
            None => {
                if ui.button("Load Rom").clicked() {
                    if let Some(rom) = try_load_rom() {
                        self.console = Some(Console::new(&rom));
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
