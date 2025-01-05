use eframe::{egui, egui_wgpu, wgpu};

use super::VirtualGpuResources;

pub struct VirtualGpuCallback;

impl egui_wgpu::CallbackTrait for VirtualGpuCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        _resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        let resource: &VirtualGpuResources = resources.get().unwrap();
        render_pass.set_pipeline(&resource.pipeline);
        render_pass.draw(0..3, 0..1);
    }
}
