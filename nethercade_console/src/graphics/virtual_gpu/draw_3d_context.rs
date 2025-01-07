use bytemuck::cast_slice;
use glam::{Mat4, Vec4Swizzles};

use super::{lights::Light, pipeline::Pipeline, virtual_render_pass::Command, VirtualGpu};

impl VirtualGpu {
    pub fn draw_tri_list(&mut self, data: &[f32], pipeline: Pipeline) {
        let attribute_count = pipeline.get_attribute_count();
        let total_attributes = data.len();
        let vertex_count = total_attributes / attribute_count;

        if total_attributes % attribute_count != 0 {
            println!("Invalid triangle list, size mismatch");
            return;
        }

        self.queue.write_buffer(
            &self.immediate_renderer.buffer,
            self.virtual_render_pass.immediate_buffer_last_index,
            bytemuck::cast_slice(data),
        );

        self.virtual_render_pass
            .commands
            .push(Command::SetPipeline(pipeline));
        self.virtual_render_pass
            .commands
            .push(Command::Draw(vertex_count as u32));
        self.virtual_render_pass.immediate_buffer_last_index += total_attributes as u64 * 4;
    }

    // TOOD: Write this
    pub fn draw_tri_list_indexed(&mut self, data: &[f32], indices: &[i16], pipeline: Pipeline) {}

    pub fn push_light(&mut self, light: &Light) {
        let offset = self.virtual_render_pass.light_count * size_of::<Light>() as u64;
        let mut light = *light;
        let view_position = self.camera.get_view() * light.position_range.xyz().extend(1.0);
        let view_direction = self.camera.get_view() * light.direction_min_angle.xyz().extend(0.0);

        light.position_range = view_position.xyz().extend(light.position_range.w);
        light.direction_min_angle = view_direction.xyz().extend(light.direction_min_angle.w);

        self.queue.write_buffer(
            &self.lights.buffer,
            offset,
            cast_slice(&light.get_light_uniforms()),
        );

        self.virtual_render_pass.light_count += 1;
    }

    pub fn push_matrix(&mut self, matrix: Mat4) {
        let offset = self.virtual_render_pass.inistance_count * size_of::<Mat4>() as u64;
        self.queue
            .write_buffer(&self.instance_buffer, offset, bytemuck::bytes_of(&matrix));
        self.virtual_render_pass
            .commands
            .push(Command::SetModelMatrix);
        self.virtual_render_pass.inistance_count += 1;
    }

    pub fn draw_static_mesh(&mut self, index: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::DrawStaticMesh(index))
    }

    pub fn draw_static_mesh_indexed(&mut self, index: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::DrawStaticMeshIndexed(index))
    }

    pub fn draw_sprite(&mut self, index: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::DrawSprite(index));
    }

    pub fn set_texture(&mut self, tex_id: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::SetTexture(tex_id));
    }
}
