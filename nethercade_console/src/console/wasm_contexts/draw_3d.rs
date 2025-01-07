use std::sync::Arc;

use bytemuck::cast_slice;
use eframe::wgpu;
use nethercade_core::Rom;
use wasmtime::{Caller, Linker};

use crate::graphics::{pipeline::Pipeline, VirtualGpu};

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
            .func_wrap(
                "env",
                "draw_tri_list",
                |mut caller: Caller<WasmContexts>, a: i32, b: i32, c: i32| {
                    let pipeline = Pipeline::try_from(c).unwrap();
                    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
                    let (data, store) = mem.data_and_store_mut(&mut caller);
                    let data: &[f32] = cast_slice(&data[a as usize..]);
                    store
                        .draw_3d
                        .vgpu
                        .draw_tri_list(&data[..b as usize], pipeline);
                    Ok(())
                },
            )
            .unwrap();

        // TODO: draw_tri_list_indexed
        // TODO: push_light
        // TODO: push_matrix
        // TODO: draw_static_mesh
        // TODO: draw_static_mesh_indexed
        // TODO: draw_sprite
        // TODO: set_texture

        // TODO: Add logic which only allows these during init.
        // Loading
        // TODO: load_texture
        // TODO: load_static_mesh
        // TODO: load_static_mesh_indexed
    }
}
