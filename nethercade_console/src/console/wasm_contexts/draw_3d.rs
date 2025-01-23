use std::{cell::RefCell, rc::Rc};

use bytemuck::{bytes_of, cast_slice, from_bytes};
use glam::{Mat4, Vec3, Vec4};
use wasmtime::{Caller, Linker};

use crate::graphics::{
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
        linker
            .func_wrap("env", "push_model_matrix", push_model_matrix)
            .unwrap();
        linker
            .func_wrap("env", "push_proj_matrix", push_proj_matrix)
            .unwrap();
        linker
            .func_wrap("env", "push_view_matrix_pos", push_view_matrix_pos)
            .unwrap();
        linker
            .func_wrap("env", "draw_static_mesh", draw_static_mesh)
            .unwrap();
        linker
            .func_wrap("env", "draw_static_mesh_indexed", draw_static_mesh_indexed)
            .unwrap();
        linker.func_wrap("env", "draw_sprite", draw_sprite).unwrap();
        linker.func_wrap("env", "set_texture", set_texture).unwrap();
        linker.func_wrap("env", "set_matcap", set_matcap).unwrap();
        linker
            .func_wrap("env", "clear_textures", clear_textures)
            .unwrap();

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
            &self.vgpu.borrow().immediate_renderer.vertex_buffer,
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

    pub fn push_model_matrix(&mut self, model: Mat4) {
        let offset_model = self.vrp.model_matrix_count * size_of::<Mat4>() as u64;
        self.vgpu.borrow().queue.write_buffer(
            &self.vgpu.borrow().immediate_renderer.model_buffer,
            offset_model,
            bytes_of(&model),
        );
        self.vrp.push_model_matrix(
            &self.vgpu.borrow().instance_buffer,
            &self.vgpu.borrow().queue,
        );
    }

    pub fn push_view_matrix_pos(&mut self, view: Mat4, pos: Vec3) {
        let offset_view = self.vrp.view_pos_count * size_of::<Mat4>() as u64;
        self.vgpu.borrow().queue.write_buffer(
            &self.vgpu.borrow().immediate_renderer.view_buffer,
            offset_view,
            bytes_of(&view),
        );
        // Wrong type to correctly pad here
        let offset_pos = self.vrp.view_pos_count * size_of::<Vec4>() as u64;
        self.vgpu.borrow().queue.write_buffer(
            &self.vgpu.borrow().immediate_renderer.camera_pos_buffer,
            offset_pos,
            bytes_of(&pos),
        );
        self.vrp.push_view_pos(
            &self.vgpu.borrow().instance_buffer,
            &self.vgpu.borrow().queue,
        );
    }

    pub fn push_projection_matrix(&mut self, proj: Mat4) {
        let offset_proj = self.vrp.projection_matrix_count * size_of::<Mat4>() as u64;
        self.vgpu.borrow().queue.write_buffer(
            &self.vgpu.borrow().immediate_renderer.proj_buffer,
            offset_proj,
            bytes_of(&proj),
        );
        self.vrp.push_proj_matrix(
            &self.vgpu.borrow().instance_buffer,
            &self.vgpu.borrow().queue,
        );
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

    pub fn set_texture(&mut self, tex_id: usize, layer: usize, blend_mode: usize) {
        self.vrp
            .commands
            .push(Command::SetTexture(tex_id, layer, blend_mode));
    }

    pub fn set_matcap(&mut self, tex_id: usize, layer: usize, blend_mode: usize) {
        self.vrp
            .commands
            .push(Command::SetMatcap(tex_id, layer, blend_mode));
    }

    pub fn clear_textures(&mut self) {
        self.vrp.commands.push(Command::ClearTextures);
    }

    fn load_texture(&mut self, data: &[u8], width: u32, height: u32, has_alpha: bool) -> i32 {
        self.vgpu
            .borrow_mut()
            .load_texture_raw(data, width, height, has_alpha) as i32
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

fn push_model_matrix(mut caller: Caller<WasmContexts>, mat_ptr: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called push_model_matrix outside of draw.");
        return;
    }

    let mat_ptr = mat_ptr as usize;
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let mat: &Mat4 = from_bytes(&data[mat_ptr..mat_ptr + size_of::<Mat4>()]);
    store.draw_3d.push_model_matrix(*mat);
}

fn push_view_matrix_pos(mut caller: Caller<WasmContexts>, view_ptr: i32, pos_ptr: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called push_model_matrix outside of draw.");
        return;
    }

    let view_ptr = view_ptr as usize;
    let pos_ptr = pos_ptr as usize;
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let mat: &Mat4 = from_bytes(&data[view_ptr..view_ptr + size_of::<Mat4>()]);
    let pos: &Vec3 = from_bytes(&data[pos_ptr..pos_ptr + size_of::<Vec3>()]);
    store.draw_3d.push_view_matrix_pos(*mat, *pos);
}

fn push_proj_matrix(mut caller: Caller<WasmContexts>, proj_ptr: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called push_proj_matrix outside of draw.");
        return;
    }

    let proj_ptr = proj_ptr as usize;
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let mat: &Mat4 = from_bytes(&data[proj_ptr..proj_ptr + size_of::<Mat4>()]);
    store.draw_3d.push_projection_matrix(*mat);
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

fn set_texture(mut caller: Caller<WasmContexts>, tex_id: i32, layer: i32, blend_mode: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called set_texture outside of draw.");
        return;
    }
    caller
        .data_mut()
        .draw_3d
        .set_texture(tex_id as usize, layer as usize, blend_mode as usize);
}

fn set_matcap(mut caller: Caller<WasmContexts>, tex_id: i32, layer: i32, blend_mode: i32) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called set_matcap outside of draw.");
        return;
    }
    caller
        .data_mut()
        .draw_3d
        .set_matcap(tex_id as usize, layer as usize, blend_mode as usize);
}

fn clear_textures(mut caller: Caller<WasmContexts>) {
    if caller.data().draw_3d.state != DrawContextState::Draw {
        println!("Called clear_textures outside of draw.");
        return;
    }
    caller.data_mut().draw_3d.clear_textures();
}

fn load_texture(
    mut caller: Caller<WasmContexts>,
    data_ptr: i32,
    width: i32,
    height: i32,
    has_alpha: i32,
) -> i32 {
    if caller.data().draw_3d.state != DrawContextState::Init {
        println!("Called load_texture outside of init.");
        return -1;
    }

    let has_alpha = has_alpha != 0;
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);

    let byte_count = width * height * if has_alpha { 4 } else { 3 };

    let data: &[u8] = &data[data_ptr as usize..(data_ptr + byte_count) as usize];
    store
        .draw_3d
        .load_texture(data, width as u32, height as u32, has_alpha)
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
