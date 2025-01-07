use bytemuck::cast_slice;
use eframe::wgpu;

use super::mesh;

pub struct QuadRenderer {
    pub quad_vertex_buffer: wgpu::Buffer,
    pub quad_index_buffer: wgpu::Buffer,
}

impl QuadRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let quad_vertex_buffer = device.create_buffer(&mesh::quad_vertex_buffer_descriptor());
        queue.write_buffer(&quad_vertex_buffer, 0, cast_slice(mesh::quad_vertices()));

        let quad_index_buffer = device.create_buffer(&mesh::index_buffer_descriptor(
            size_of::<[u16; 6]>() as u64,
            Some("Quad Index Buffer"),
        ));
        queue.write_buffer(&quad_index_buffer, 0, cast_slice(mesh::quad_indices()));

        Self {
            quad_vertex_buffer,
            quad_index_buffer,
        }
    }
}
