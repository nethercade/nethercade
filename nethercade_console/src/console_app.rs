use std::{ffi::OsStr, io::Read};

use eframe::{
    egui::{self, Sense},
    egui_wgpu,
};
use gilrs::Gilrs;
use nethercade_core::Rom;

use crate::{
    console::{Console, LocalInputManager, LocalPlayerId, MouseEventCollector},
    graphics::VirtualGpuCallback,
};

pub struct ConsoleApp {
    console: Console,
    input_manager: LocalInputManager,
    gilrs: Gilrs,
}

impl ConsoleApp {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        // TODO: Preload whatever stuff needed
        let wgpu_render_state = cc.wgpu_render_state.as_ref()?;
        let queue = wgpu_render_state.queue.clone();

        let device = wgpu_render_state.device.clone();
        let format = wgpu_render_state.target_format;

        let console = Console::new(&device, &queue, format);
        let frame_buffer = console.get_frame_buffer();

        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(frame_buffer);

        Some(Self {
            console,
            input_manager: LocalInputManager::new(),
            gilrs: Gilrs::new().unwrap(),
        })
    }
}

impl eframe::App for ConsoleApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // TODO: Render a File Menu
        egui::CentralPanel::default().show(ctx, |ui| match &mut self.console.game {
            Some(game) => {
                // Pre Update Input
                // TODO: Clean this up
                let held_keys = ctx.input(|i| i.keys_down.clone());
                
                let net_state = self.input_manager.generate_input_state(
                    LocalPlayerId(0),
                    &MouseEventCollector::default(),
                    &held_keys,
                    &self.gilrs,
                );

                game.store.data_mut().input.input_entries[0].current = net_state.input_state;
                game.store.data_mut().input.input_entries[0].current_mouse = net_state.mouse_state;

                game.update();
                game.draw();

                // Post Update Input
                // TODO: Clean this up
                game.store
                    .data_mut()
                    .input
                    .input_entries
                    .iter_mut()
                    .for_each(|inputs| {
                        inputs.previous = inputs.current.buttons;
                        inputs.previous_mouse = inputs.current_mouse;
                    });
                

                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let (width, height) = game.rom.resolution.dimensions();
                    let (rect, response) = ui.allocate_exact_size(
                        egui::Vec2::new(width as f32, height as f32),
                        Sense::click(),
                    );

                    // TODO: Feed this into mouse input
                    if let Some(hover) = response.hover_pos() {
                        let _pos = hover - response.interact_rect.left_top();
                        // This is the mouse position
                    }

                    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                        rect,
                        VirtualGpuCallback,
                    ));
                });
            }
            None => {
                if ui.button("Load Rom").clicked() {
                    if let Some(rom) = try_load_rom() {
                        // TODO: Add more players
                        self.console.load_rom(rom, self.console.vgpu.clone(), 1);
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
