use bytemuck::bytes_of;
use eframe::wgpu;

use super::{pipeline::Pipeline, VirtualGpu, TEXTURE_BIND_GROUP_INDEX, VERTEX_BUFFER_INDEX};
pub struct VirtualRenderPass {
    pub commands: Vec<Command>,

    pub immediate_buffer_last_index: u64,
    pub instance_count: u64,

    pub light_count: u64,
    pub model_matrix_count: u64,
    pub view_pos_count: u64,
    pub projection_matrix_count: u64,
}

pub enum Command {
    SetPipeline(Pipeline),
    Draw(u32),                       //Vertex Count
    SetTexture(usize, usize, usize), // TextureId, Layer Index, Blend Mode
    SetMatcap(usize, usize, usize),  // Matcap Id, Layer Index, Blend Mode
    ClearTextures,
    UpdateInstance,
    DrawStaticMesh(usize),        // Static Mesh ID
    DrawStaticMeshIndexed(usize), // Static Mesh Indexed Id
    DrawSprite(usize),
}

#[derive(Default)]
struct TextureStates {
    texture_indices: [usize; 4],
    blend_modes: [u8; 4],
    is_matcap: [bool; 4],
}

impl TextureStates {
    // TODO: Use this!
    fn to_push_constants(&self) -> [u8; 8] {
        let mut out = [0; 8];

        out[..4].copy_from_slice(&self.blend_modes);

        // Pack the is_matcap array into a single byte
        let mut matcap_mask = 0u32;
        for (i, &is_matcap) in self.is_matcap.iter().enumerate() {
            if is_matcap {
                matcap_mask |= 1 << i; // Set the corresponding bit
            }
        }

        // Store the matcap mask in the last byte
        out[4..8].copy_from_slice(bytemuck::bytes_of(&matcap_mask));

        out
    }

    fn set_texture(&mut self, index: usize, layer: usize, blend_mode: usize, is_matcap: bool) {
        self.texture_indices[layer] = index;
        self.blend_modes[layer] = blend_mode as u8;
        self.is_matcap[layer] = is_matcap
    }

    fn create_bind_group(&self, gpu: &VirtualGpu) -> wgpu::BindGroup {
        gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &gpu.textures.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &gpu.textures.textures[self.texture_indices[0]].view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &gpu.textures.textures[self.texture_indices[1]].view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(
                        &gpu.textures.textures[self.texture_indices[2]].view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(
                        &gpu.textures.textures[self.texture_indices[3]].view,
                    ),
                },
            ],
        })
    }
}

impl VirtualRenderPass {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            instance_count: 0,
            light_count: 0,
            immediate_buffer_last_index: 0,
            model_matrix_count: 0,
            view_pos_count: 0,
            projection_matrix_count: 0,
        }
    }

    fn get_instance_data(&self) -> Option<[u32; 4]> {
        if self.model_matrix_count == 0
            || self.view_pos_count == 0
            || self.projection_matrix_count == 0
        {
            None
        } else {
            Some([
                self.model_matrix_count as u32 - 1,
                self.view_pos_count as u32 - 1,
                self.projection_matrix_count as u32 - 1,
                self.light_count as u32,
            ])
        }
    }

    fn write_buffer(&mut self, instance_buffer: &wgpu::Buffer, queue: &wgpu::Queue) {
        let offset = self.instance_count * size_of::<[u32; 4]>() as u64;
        if let Some(data) = self.get_instance_data() {
            queue.write_buffer(instance_buffer, offset, bytes_of(&data));
            self.instance_count += 1;
            self.commands.push(Command::UpdateInstance);
        }
    }

    pub fn push_model_matrix(&mut self, instance_buffer: &wgpu::Buffer, queue: &wgpu::Queue) {
        self.model_matrix_count += 1;
        self.write_buffer(instance_buffer, queue);
    }

    pub fn push_view_pos(&mut self, instance_buffer: &wgpu::Buffer, queue: &wgpu::Queue) {
        self.view_pos_count += 1;
        self.write_buffer(instance_buffer, queue);
    }

    pub fn push_proj_matrix(&mut self, instance_buffer: &wgpu::Buffer, queue: &wgpu::Queue) {
        self.projection_matrix_count += 1;
        self.write_buffer(instance_buffer, queue);
    }

    pub fn reset(&mut self) {
        self.commands.clear();
        self.instance_count = 0;
        self.light_count = 0;
        self.immediate_buffer_last_index = 0;
        self.model_matrix_count = 0;
        self.view_pos_count = 0;
        self.projection_matrix_count = 0;
    }

    pub fn execute(&self, rp: &mut wgpu::RenderPass, gpu: &VirtualGpu) {
        if self.instance_count == 0 {
            println!("Matrices are invalid, please set model, view, and projection.");
            return;
        }

        let mut current_byte_index = 0;
        let mut current_vertex_size = 0;
        let mut current_instance = 0;

        let mut texture_state = TextureStates::default();

        rp.set_bind_group(
            TEXTURE_BIND_GROUP_INDEX,
            &texture_state.create_bind_group(gpu),
            &[],
        );

        for command in self.commands.iter() {
            match command {
                Command::SetPipeline(pipeline) => {
                    rp.set_pipeline(&gpu.render_pipelines[pipeline.get_shader()]);
                    rp.set_push_constants(
                        wgpu::ShaderStages::FRAGMENT,
                        0,
                        &texture_state.to_push_constants(),
                    );
                    current_vertex_size = pipeline.get_vertex_size();
                }
                Command::Draw(vertex_count) => {
                    rp.set_vertex_buffer(
                        VERTEX_BUFFER_INDEX,
                        gpu.immediate_renderer
                            .vertex_buffer
                            .slice(current_byte_index..),
                    );
                    rp.draw(0..*vertex_count, current_instance - 1..current_instance);
                    current_byte_index += *vertex_count as u64 * current_vertex_size as u64;
                }
                Command::SetTexture(tex_index, layer_index, blend_mode) => {
                    texture_state.set_texture(*tex_index, *layer_index, *blend_mode, false);
                    rp.set_bind_group(
                        TEXTURE_BIND_GROUP_INDEX,
                        &texture_state.create_bind_group(gpu),
                        &[],
                    );
                }
                Command::SetMatcap(matcap_index, layer_index, blend_mode) => {
                    texture_state.set_texture(*matcap_index, *layer_index, *blend_mode, true);
                    rp.set_bind_group(
                        TEXTURE_BIND_GROUP_INDEX,
                        &texture_state.create_bind_group(gpu),
                        &[],
                    );
                }
                Command::ClearTextures => {
                    texture_state = TextureStates::default();
                    rp.set_bind_group(
                        TEXTURE_BIND_GROUP_INDEX,
                        &texture_state.create_bind_group(gpu),
                        &[],
                    );
                }
                Command::DrawStaticMesh(index) => {
                    let mesh = &gpu.preloaded_renderer.meshes[*index];
                    rp.set_pipeline(&gpu.render_pipelines[mesh.pipeline.get_shader()]);
                    rp.set_push_constants(
                        wgpu::ShaderStages::FRAGMENT,
                        0,
                        &texture_state.to_push_constants(),
                    );
                    rp.set_vertex_buffer(VERTEX_BUFFER_INDEX, mesh.vertex_buffer.slice(..));
                    rp.draw(0..mesh.vertex_count, current_instance - 1..current_instance);
                }
                Command::DrawStaticMeshIndexed(index) => {
                    let mesh = &gpu.preloaded_renderer.indexed_meshes[*index];
                    rp.set_pipeline(&gpu.render_pipelines[mesh.pipeline.get_shader()]);
                    rp.set_push_constants(
                        wgpu::ShaderStages::FRAGMENT,
                        0,
                        &texture_state.to_push_constants(),
                    );
                    rp.set_vertex_buffer(VERTEX_BUFFER_INDEX, mesh.vertex_buffer.slice(..));
                    rp.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    rp.draw_indexed(
                        0..mesh.index_count,
                        0,
                        current_instance - 1..current_instance,
                    );
                }
                Command::DrawSprite(_sprite_index) => {
                    todo!()
                    // let texture = &gpu.textures.textures[*sprite_index];
                    // rp.set_pipeline(&gpu.render_pipelines[Pipeline::Quad2d.get_shader()]);
                    // rp.set_bind_group(TEXTURE_BIND_GROUP_INDEX, &texture.bind_group, &[]);
                    // rp.set_index_buffer(
                    //     gpu.quad_renderer.quad_index_buffer.slice(..),
                    //     wgpu::IndexFormat::Uint16,
                    // );
                    // rp.set_vertex_buffer(
                    //     VERTEX_BUFFER_INDEX,
                    //     gpu.quad_renderer.quad_vertex_buffer.slice(..),
                    // );
                    // rp.draw_indexed(0..6, 0, current_instance - 1..current_instance)
                }
                Command::UpdateInstance => {
                    current_instance += 1;
                }
            }
        }
    }
}
