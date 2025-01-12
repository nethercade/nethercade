use eframe::wgpu;

use super::{pipeline::Pipeline, VirtualGpu, TEXTURE_BIND_GROUP_INDEX, VERTEX_BUFFER_INDEX};
pub struct VirtualRenderPass {
    pub commands: Vec<Command>,

    pub immediate_buffer_last_index: u64,
    pub instance_count: u64,

    pub light_count: u64,
}

pub enum Command {
    SetPipeline(Pipeline),
    Draw(u32),         //Vertex Count
    SetTexture(usize), // TextureId
    SetModelMatrix,
    DrawStaticMesh(usize),        // Static Mesh ID
    DrawStaticMeshIndexed(usize), // Static Mesh Indexed Id
    DrawSprite(usize),
}

impl VirtualRenderPass {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            instance_count: 0,
            light_count: 0,
            immediate_buffer_last_index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.commands.clear();
        self.instance_count = 0;
        self.light_count = 0;
        self.immediate_buffer_last_index = 0;
    }

    pub fn execute(&self, rp: &mut wgpu::RenderPass, gpu: &VirtualGpu) {
        let mut current_byte_index = 0;
        let mut current_vertex_size = 0;
        let mut current_model_matrix = 0;

        for command in self.commands.iter() {
            match command {
                Command::SetPipeline(pipeline) => {
                    rp.set_pipeline(&gpu.render_pipelines[pipeline.get_shader()]);
                    current_vertex_size = pipeline.get_vertex_size();
                }
                Command::Draw(vertex_count) => {
                    rp.set_vertex_buffer(
                        VERTEX_BUFFER_INDEX,
                        gpu.immediate_renderer.buffer.slice(current_byte_index..),
                    );
                    rp.draw(
                        0..*vertex_count,
                        current_model_matrix - 1..current_model_matrix,
                    );
                    current_byte_index += *vertex_count as u64 * current_vertex_size as u64;
                }
                Command::SetTexture(tex_index) => {
                    let texture = &gpu.textures.textures[*tex_index];
                    rp.set_bind_group(TEXTURE_BIND_GROUP_INDEX, &texture.bind_group, &[]);
                }
                Command::SetModelMatrix => {
                    current_model_matrix += 1;
                }
                Command::DrawStaticMesh(index) => {
                    let mesh = &gpu.preloaded_renderer.meshes[*index];
                    rp.set_pipeline(&gpu.render_pipelines[mesh.pipeline.get_shader()]);
                    rp.set_vertex_buffer(VERTEX_BUFFER_INDEX, mesh.vertex_buffer.slice(..));
                    rp.draw(
                        0..mesh.vertex_count,
                        current_model_matrix - 1..current_model_matrix,
                    );
                }
                Command::DrawStaticMeshIndexed(index) => {
                    let mesh = &gpu.preloaded_renderer.indexed_meshes[*index];
                    rp.set_pipeline(&gpu.render_pipelines[mesh.pipeline.get_shader()]);
                    rp.set_vertex_buffer(VERTEX_BUFFER_INDEX, mesh.vertex_buffer.slice(..));
                    rp.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    rp.draw_indexed(
                        0..mesh.index_count,
                        0,
                        current_model_matrix - 1..current_model_matrix,
                    );
                }
                Command::DrawSprite(sprite_index) => {
                    let texture = &gpu.textures.textures[*sprite_index];
                    rp.set_pipeline(&gpu.render_pipelines[Pipeline::Quad2d.get_shader()]);
                    rp.set_bind_group(TEXTURE_BIND_GROUP_INDEX, &texture.bind_group, &[]);
                    rp.set_index_buffer(
                        gpu.quad_renderer.quad_index_buffer.slice(..),
                        wgpu::IndexFormat::Uint16,
                    );
                    rp.set_vertex_buffer(
                        VERTEX_BUFFER_INDEX,
                        gpu.quad_renderer.quad_vertex_buffer.slice(..),
                    );
                    rp.draw_indexed(0..6, 0, current_model_matrix - 1..current_model_matrix)
                }
            }
        }
    }
}
