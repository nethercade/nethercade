use std::sync::Arc;

use bytemuck::{cast_slice, from_bytes};
use eframe::wgpu;
use glam::Mat4;
use nethercade_core::Rom;
use wasmtime::{Caller, Linker};

use crate::graphics::{lights::Light, pipeline::Pipeline, VirtualGpu};

use super::WasmContexts;

// TODO: This could likely be replaced with just a VRenderPass to save lots
// of time during startup
pub struct Draw3dContext {
    pub vgpu: VirtualGpu,
}

impl Draw3dContext {
    pub fn new(
        rom: &Rom,
        device: &Arc<wgpu::Device>,
        queue: &Arc<wgpu::Queue>,
        format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            vgpu: VirtualGpu::new(rom, device, queue, format),
        }
    }

    pub fn link(linker: &mut Linker<WasmContexts>) {
        // TODO: Add logic which only allows these during drawing.
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

        // TODO: Add logic which only allows these during init.
        // Loading
        // TODO: load_texture
        // TODO: load_static_mesh
        // TODO: load_static_mesh_indexed
    }
}

fn draw_tri_list(mut caller: Caller<WasmContexts>, data_ptr: i32, len: i32, pipeline: i32) {
    let pipeline = Pipeline::try_from(pipeline).unwrap();
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let data: &[f32] = cast_slice(&data[data_ptr as usize..]);
    store
        .draw_3d
        .vgpu
        .draw_tri_list(&data[..len as usize], pipeline);
}

fn draw_tri_list_indexed(
    mut caller: Caller<WasmContexts>,
    data_ptr: i32,
    data_len: i32,
    index_ptr: i32,
    index_len: i32,
    pipeline: i32,
) {
    // TODO: Write this
}

fn push_light(mut caller: Caller<WasmContexts>, light_ptr: i32) {
    let light_ptr = light_ptr as usize;
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let light: &Light = from_bytes(&data[light_ptr..light_ptr + size_of::<Light>()]);
    store.draw_3d.vgpu.push_light(light);
}

fn push_matrix(mut caller: Caller<WasmContexts>, mat_ptr: i32) {
    let mat_ptr = mat_ptr as usize;
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let mat: &Mat4 = from_bytes(&data[mat_ptr..mat_ptr + size_of::<Mat4>()]);
    store.draw_3d.vgpu.push_matrix(*mat);
}

fn draw_static_mesh(mut caller: Caller<WasmContexts>, id: i32) {
    caller.data_mut().draw_3d.vgpu.draw_static_mesh(id as usize);
}

fn draw_static_mesh_indexed(mut caller: Caller<WasmContexts>, id: i32) {
    caller
        .data_mut()
        .draw_3d
        .vgpu
        .draw_static_mesh_indexed(id as usize);
}

fn draw_sprite(mut caller: Caller<WasmContexts>, sprite_id: i32) {
    caller
        .data_mut()
        .draw_3d
        .vgpu
        .draw_sprite(sprite_id as usize);
}

fn set_texture(mut caller: Caller<WasmContexts>, tex_id: i32) {
    caller.data_mut().draw_3d.vgpu.set_texture(tex_id as usize);
}
