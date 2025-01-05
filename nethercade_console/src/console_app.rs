use eframe::{
    egui::{self, Sense},
    egui_wgpu,
};

use crate::graphics::*;

pub struct ConsoleApp {
    frame_count: usize,
}

impl Default for ConsoleApp {
    fn default() -> Self {
        Self { frame_count: 0 }
    }
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

        egui::CentralPanel::default().show(ctx, |ui| {
            self.frame_count += 1;
            ui.label("Application");
            ui.label(format!("{}", self.frame_count));

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (rect, _response) =
                    ui.allocate_exact_size(egui::Vec2::splat(300.0), Sense::click());

                ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                    rect,
                    VirtualGpuCallback,
                ));
            });
        });

        // Render continiously
        ctx.request_repaint();
    }
}
