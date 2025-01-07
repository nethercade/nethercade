use eframe::wgpu;

use super::mesh;

// TODO: Could do something for immediate Textures

pub struct ImmediateRenderer {
    pub buffer: wgpu::Buffer,
}

impl ImmediateRenderer {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer(&mesh::vertex_buffer_descriptor(
            1024 * 1024 * 8, // 8mb
            Some("Immediate Vertex Buffer"),
        ));

        Self { buffer }
    }
}
