use bytemuck::cast_slice;
use eframe::wgpu;

use super::{
    mesh::{self, IndexedMesh, Mesh},
    pipeline::Pipeline,
};

pub struct PreloadedRenderer {
    pub meshes: Vec<Mesh>,
    pub indexed_meshes: Vec<IndexedMesh>,
}

impl PreloadedRenderer {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            indexed_meshes: Vec::new(),
        }
    }

    pub fn load_static_mesh(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[f32],
        pipeline: Pipeline,
    ) -> usize {
        let attribute_count = pipeline.get_attribute_count();
        let total_attributes = data.len();
        let vertex_count = total_attributes / attribute_count;
        let bytes = vertex_count * attribute_count * 4;

        if total_attributes % attribute_count != 0 {
            panic!(
                "Invalid mesh list, size mismatch for: {pipeline:?}. Received {total_attributes}, expected multiple of {attribute_count}."
            );
        }

        let vertex_buffer =
            device.create_buffer(&mesh::vertex_buffer_descriptor(bytes as u64, None));

        queue.write_buffer(&vertex_buffer, 0, cast_slice(data));

        let mesh = Mesh {
            vertex_buffer,
            pipeline,
            vertex_count: vertex_count as u32,
        };

        self.meshes.push(mesh);
        self.meshes.len() - 1
    }

    pub fn load_static_mesh_indexed(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[f32],
        indices: &[u16],
        pipeline: Pipeline,
    ) -> usize {
        let attribute_count = pipeline.get_attribute_count();
        let total_attributes = data.len();
        let vertex_count = total_attributes / attribute_count;
        let bytes = vertex_count * attribute_count * 4;

        if total_attributes % attribute_count != 0 {
            panic!("Invalid mesh list, size mismatch");
        }

        let vertex_buffer =
            device.create_buffer(&mesh::vertex_buffer_descriptor(bytes as u64, None));

        let bytes = std::mem::size_of_val(indices);
        let index_buffer = device.create_buffer(&mesh::index_buffer_descriptor(bytes as u64, None));

        queue.write_buffer(&vertex_buffer, 0, cast_slice(data));
        queue.write_buffer(&index_buffer, 0, cast_slice(indices));

        let mesh = IndexedMesh {
            vertex_buffer,
            index_buffer,
            pipeline,
            index_count: indices.len() as u32,
        };

        self.indexed_meshes.push(mesh);
        self.indexed_meshes.len() - 1
    }
}
