use std::sync::Arc;

use eframe::{egui, egui_wgpu, wgpu};

use super::frame_buffer::{FrameBuffer, FRAME_BUFFER_BIND_GROUP_INDEX};

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
        let frame_buffer: &Arc<FrameBuffer> = resources.get().unwrap();
        render_pass.set_pipeline(&frame_buffer.pipeline);
        render_pass.set_bind_group(
            FRAME_BUFFER_BIND_GROUP_INDEX,
            &frame_buffer.texture_bind_group,
            &[],
        );
        render_pass.draw(0..4, 0..1);
    }
}
