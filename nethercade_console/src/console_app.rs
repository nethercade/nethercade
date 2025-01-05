use eframe::egui;

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
        let wgpu_render_state = cc.wgpu_render_state.as_ref()?;

        Some(Self::default())
    }
}

impl eframe::App for ConsoleApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.frame_count += 1;
            ui.label("Application");
            ui.label(format!("{}", self.frame_count));
        });
    }
}
