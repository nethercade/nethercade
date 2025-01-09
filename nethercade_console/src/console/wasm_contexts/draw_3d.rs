use std::{cell::RefCell, rc::Rc};

use bytemuck::{cast_slice, from_bytes};
use glam::{Mat4, Vec4Swizzles};
use wasmtime::{Caller, Linker};

use crate::graphics::{
    lights::Light,
    pipeline::Pipeline,
    virtual_render_pass::{Command, VirtualRenderPass},
    VirtualGpu,
};

use super::WasmContexts;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DrawContextState {
    Init,
    Draw,
    Invalid,
}

pub struct Draw3dContext {
    pub vrp: VirtualRenderPass,
    pub vgpu: Rc<RefCell<VirtualGpu>>,
    pub state: DrawContextState,
}

impl Draw3dContext {
    pub fn new(vgpu: Rc<RefCell<VirtualGpu>>) -> Self {
        Self {
            vrp: VirtualRenderPass::new(),
            vgpu,
            state: DrawContextState::Invalid,
        }
    }

    pub fn link(linker: &mut Linker<WasmContexts>) {
        // Drawing
        linker
            .func_wrap("env", "draw_tri_list", draw_tri_list)
            .unwrap();
        linker
            .func_wrap("env", "draw_tri_list_indexed", draw_tri_list_indexed)
            .unwrap();
        linker.func_wrap("env", "push_light", push_light).unwrap();
        linker.func_wrap("env", "push_matrix", push_matrix).unwrap();
        linker
            .func_wrap("env", "draw_static_mesh", draw_static_mesh)
            .unwrap();
        linker
            .func_wrap("env", "draw_static_mesh_indexed", draw_static_mesh_indexed)
            .unwrap();
        linker.func_wrap("env", "draw_sprite", draw_sprite).unwrap();
        linker.func_wrap("env", "set_texture", set_texture).unwrap();

        // Loading
        linker
            .func_wrap("env", "load_texture", load_texture)
            .unwrap();
        linker
            .func_wrap("env", "load_static_mesh", load_static_mesh)
            .unwrap();
        linker
            .func_wrap("env", "load_static_mesh_indexed", load_static_mesh_indexed)
            .unwrap();
    }

    pub fn draw_tri_list(&mut self, data: &[f32], pipeline: Pipeline) {
        let attribute_count = pipeline.get_attribute_count();
        let total_attributes = data.len();
        let vertex_count = total_attributes / attribute_count;

        if total_attributes % attribute_count != 0 {
            println!("Invalid triangle list, size mismatch");
            return;
        }

        self.vgpu.borrow().queue.write_buffer(
            &self.vgpu.borrow().immediate_renderer.buffer,
            self.vrp.immediate_buffer_last_index,
            bytemuck::cast_slice(data),
        );

        self.vrp.commands.push(Command::SetPipeline(pipeline));
        self.vrp.commands.push(Command::Draw(vertex_count as u32));
        self.vrp.immediate_buffer_last_index += total_attributes as u64 * 4;
    }

    // TOOD: Write this
    pub fn draw_tri_list_indexed(&mut self, _data: &[f32], _indices: &[i16], _pipeline: Pipeline) {
        todo!()
    }

    pub fn push_light(&mut self, light: &Light) {
        let offset = self.vrp.light_count * size_of::<Light>() as u64;
        let mut light = *light;
        let view_position =
            self.vgpu.borrow().camera.get_view() * light.position_range.xyz().extend(1.0);
        let view_direction =
            self.vgpu.borrow().camera.get_view() * light.direction_min_angle.xyz().extend(0.0);

        light.position_range = view_position.xyz().extend(light.position_range.w);
        light.direction_min_angle = view_direction.xyz().extend(light.direction_min_angle.w);

        self.vgpu.borrow().queue.write_buffer(
            &self.vgpu.borrow().lights.buffer,
            offset,
            cast_slice(&light.get_light_uniforms()),
        );

        self.vrp.light_count += 1;
    }

    pub fn push_matrix(&mut self, matrix: Mat4) {
        let offset = self.vrp.inistance_count * size_of::<Mat4>() as u64;
        self.vgpu.borrow().queue.write_buffer(
            &self.vgpu.borrow().instance_buffer,
            offset,
            bytemuck::bytes_of(&matrix),
        );
        self.vrp.commands.push(Command::SetModelMatrix);
        self.vrp.inistance_count += 1;
    }

    pub fn draw_static_mesh(&mut self, index: usize) {
        self.vrp.commands.push(Command::DrawStaticMesh(index))
    }

    pub fn draw_static_mesh_indexed(&mut self, index: usize) {
        self.vrp
            .commands
            .push(Command::DrawStaticMeshIndexed(index))
    }

    pub fn draw_sprite(&mut self, index: usize) {
        self.vrp.commands.push(Command::DrawSprite(index));
    }

    pub fn set_texture(&mut self, tex_id: usize) {
        self.vrp.commands.push(Command::SetTexture(tex_id));
    }

    fn load_texture(&mut self, data: &[u8], has_alpha: bool) -> i32 {
        self.vgpu.borrow_mut().load_texture_raw(data, has_alpha) as i32
    }

    fn load_static_mesh(&mut self, data: &[f32], pipeline: Pipeline) -> i32 {
        self.vgpu.borrow_mut().load_static_mesh(data, pipeline) as i32
    }

    fn load_static_mesh_indexed(
        &mut self,
        data: &[f32],
        indices: &[u16],
        pipeline: Pipeline,
    ) -> i32 {
        self.vgpu
            .borrow_mut()
            .load_static_mesh_indexed(data, indices, pipeline) as i32
    }

    pub fn render(&mut self) {
        self.vgpu.borrow_mut().render(&self.vrp);
    }
}

fn draw_tri_list(mut caller: Caller<WasmContexts>, data_ptr: i32, len: i32, pipeline: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called draw_tri_list outside of draw.");
        return;
    }

    let pipeline = Pipeline::try_from(pipeline).unwrap();
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let data: &[f32] = cast_slice(&data[data_ptr as usize..]);
    store.draw_3d.draw_tri_list(&data[..len as usize], pipeline);
}

fn draw_tri_list_indexed(
    mut caller: Caller<WasmContexts>,
    data_ptr: i32,
    data_len: i32,
    index_ptr: i32,
    index_len: i32,
    pipeline: i32,
) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called draw_tri_list_indexed outside of draw.");
        return;
    }

    let pipeline = Pipeline::try_from(pipeline).unwrap();
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let data: &[f32] = cast_slice(&data[data_ptr as usize..]);
    let index: &[i16] = cast_slice(&data[index_ptr as usize..]);
    store.draw_3d.draw_tri_list_indexed(
        &data[..data_len as usize],
        &index[..index_len as usize],
        pipeline,
    );
}

fn push_light(mut caller: Caller<WasmContexts>, light_ptr: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called push_light outside of draw.");
        return;
    }

    let light_ptr = light_ptr as usize;
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let light: &Light = from_bytes(&data[light_ptr..light_ptr + size_of::<Light>()]);
    store.draw_3d.push_light(light);
}

fn push_matrix(mut caller: Caller<WasmContexts>, mat_ptr: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called push_matrix outside of draw.");
        return;
    }

    let mat_ptr = mat_ptr as usize;
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let mat: &Mat4 = from_bytes(&data[mat_ptr..mat_ptr + size_of::<Mat4>()]);
    store.draw_3d.push_matrix(*mat);
}

fn draw_static_mesh(mut caller: Caller<WasmContexts>, id: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called draw_static_mesh outside of draw.");
        return;
    }
    caller.data_mut().draw_3d.draw_static_mesh(id as usize);
}

fn draw_static_mesh_indexed(mut caller: Caller<WasmContexts>, id: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called draw_static_mesh_indexed outside of draw.");
        return;
    }
    caller
        .data_mut()
        .draw_3d
        .draw_static_mesh_indexed(id as usize);
}

fn draw_sprite(mut caller: Caller<WasmContexts>, sprite_id: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called draw_sprite outside of draw.");
        return;
    }
    caller.data_mut().draw_3d.draw_sprite(sprite_id as usize);
}

fn set_texture(mut caller: Caller<WasmContexts>, tex_id: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called set_texture outside of draw.");
        return;
    }
    caller.data_mut().draw_3d.set_texture(tex_id as usize);
}

fn load_texture(
    mut caller: Caller<WasmContexts>,
    data_ptr: i32,
    data_len: i32,
    has_alpha: i32,
) -> i32 {
    if caller.data().draw_3d.state != DrawContextState::Init {
        println!("Called load_texture outside of init.");
        return -1;
    }

    let has_alpha = has_alpha != 0;
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let data: &[u8] = &data[data_ptr as usize..(data_ptr + data_len) as usize];
    store.draw_3d.load_texture(data, has_alpha)
}

fn load_static_mesh(
    mut caller: Caller<WasmContexts>,
    data_ptr: i32,
    data_len: i32,
    pipeline: i32,
) -> i32 {
    if caller.data().draw_3d.state != DrawContextState::Init {
        println!("Called load_static_mesh outside of init.");
        return -1;
    }

    let pipeline = Pipeline::try_from(pipeline).unwrap();
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let data: &[f32] = cast_slice(&data[data_ptr as usize..]);
    store
        .draw_3d
        .load_static_mesh(&data[..data_len as usize], pipeline)
}

fn load_static_mesh_indexed(
    mut caller: Caller<WasmContexts>,
    data_ptr: i32,
    data_len: i32,
    index_ptr: i32,
    index_len: i32,
    pipeline: i32,
) -> i32 {
    if caller.data().draw_3d.state != DrawContextState::Init {
        println!("Called load_static_mesh_indexed outside of init.");
        return -1;
    }

    let pipeline = Pipeline::try_from(pipeline).unwrap();
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let data: &[f32] = cast_slice(&data[data_ptr as usize..]);
    let index: &[u16] = cast_slice(&data[index_ptr as usize..]);
    store.draw_3d.load_static_mesh_indexed(
        &data[..data_len as usize],
        &index[..index_len as usize],
        pipeline,
    )
}
