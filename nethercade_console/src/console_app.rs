use std::{ffi::OsStr, io::Read};

use eframe::{
    egui::{self, Sense, ViewportCommand},
    egui_wgpu,
};
use egui::Vec2;
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

                // Handle Keyboard
                let held_keys = ctx.input(|i| i.keys_down.clone());
                // Handle Mouse
                let mouse_events = frame_mouse_input(ctx);

                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let (width, height) = game.rom.resolution.dimensions();
                    let width = width as f32 / ctx.pixels_per_point();
                    let height = height as f32 / ctx.pixels_per_point();

                    let available = ui.available_size();
                    let scale_x = (available.x / width).floor();
                    let scale_y = (available.y / height).floor();
                    let scale_final = scale_x.min(scale_y);

                    ctx.send_viewport_cmd(ViewportCommand::Title(format!("Scale: {scale_final}x")));

                    let (rect, response) = ui.allocate_exact_size(
                        egui::Vec2::new(width * scale_final, height * scale_final),
                        Sense::click(),
                    );

                    let mouse_pos = if let Some(hover) = response.hover_pos() {
                        let mut pos = hover - response.interact_rect.left_top();
                        pos.x = pos.x.clamp(0.0, width as f32);
                        pos.y = pos.y.clamp(0.0, height as f32);
                        Some(pos)
                    } else {
                        None
                    };

                    let net_state = self.input_manager.generate_input_state(
                        LocalPlayerId(0),
                        &mouse_events,
                        mouse_pos,
                        &held_keys,
                        &self.gilrs,
                    );

                    game.store.data_mut().input.input_entries[0].current = net_state.input_state;
                    game.store.data_mut().input.input_entries[0].current_mouse =
                        net_state.mouse_state;

                    game.update();
                    game.draw();

                    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                        rect,
                        VirtualGpuCallback,
                    ));
                });

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
            }
            None => {
                if ui.button("Load Rom").clicked() {
                    if let Some(rom) = try_load_rom() {
                        // TODO: Add more players
                        let dimensions = rom.resolution.dimensions();
                        let ppp = ctx.pixels_per_point();
                        let resolution =
                            Vec2::new(dimensions.0 as f32 / ppp, dimensions.1 as f32 / ppp);
                        let spacing = &ctx.style().spacing;
                        let new_size = resolution
                            + spacing.window_margin.sum()
                            + spacing.item_spacing
                            + spacing.menu_margin.sum();
                        ctx.send_viewport_cmd(ViewportCommand::InnerSize(new_size));
                        ctx.send_viewport_cmd(ViewportCommand::MinInnerSize(new_size));
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

fn frame_mouse_input(ctx: &eframe::egui::Context) -> MouseEventCollector {
    let mut mouse_events = MouseEventCollector::default();
    ctx.input(|i| {
        for event in &i.events {
            match event {
                egui::Event::Copy => (),
                egui::Event::Cut => (),
                egui::Event::Paste(_) => (),
                egui::Event::Text(_) => (),
                egui::Event::Key { .. } => (),
                egui::Event::MouseMoved(..) => (),
                egui::Event::PointerGone => (),
                egui::Event::Zoom(_) => (),
                egui::Event::Ime(_ime_event) => (),
                egui::Event::Touch { .. } => (),
                egui::Event::WindowFocused(_) => (),
                egui::Event::AccessKitActionRequest(..) => (),
                egui::Event::Screenshot { .. } => (),

                egui::Event::PointerButton {
                    button, pressed, ..
                } => {
                    if *pressed {
                        match button {
                            egui::PointerButton::Extra1 => (),
                            egui::PointerButton::Extra2 => (),
                            egui::PointerButton::Primary => {
                                mouse_events.button_left = true;
                            }
                            egui::PointerButton::Secondary => {
                                mouse_events.button_right = true;
                            }
                            egui::PointerButton::Middle => {
                                mouse_events.button_middle = true;
                            }
                        }
                    }
                }
                egui::Event::PointerMoved(vec2) => {
                    mouse_events.delta_x += vec2.x as i16;
                    mouse_events.delta_y += vec2.y as i16;
                }
                egui::Event::MouseWheel { delta, .. } => {
                    let dx = delta.x;
                    let dy = delta.y;

                    if dx > 0.0 {
                        mouse_events.wheel_right = true;
                    } else if dx < 0.0 {
                        mouse_events.wheel_left = true;
                    }

                    if dy > 0.0 {
                        mouse_events.wheel_down = true;
                    } else if dy < 0.0 {
                        mouse_events.wheel_up = true;
                    }
                }
            }
        }
    });

    mouse_events
}
