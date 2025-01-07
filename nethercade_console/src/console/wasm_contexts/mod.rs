use std::sync::Arc;

use eframe::wgpu;
use nethercade_core::Rom;

mod data;
use data::*;

mod draw_3d;
use draw_3d::*;

use wasmtime::Linker;

pub struct WasmContexts {
    data: DataContext,
    pub draw_3d: Draw3dContext,
}

impl WasmContexts {
    pub fn new(
        rom: &Rom,
        device: &Arc<wgpu::Device>,
        queue: &Arc<wgpu::Queue>,
        format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            data: DataContext::new(rom),
            draw_3d: Draw3dContext::new(rom, device, queue, format),
        }
    }

    pub fn link(linker: &mut Linker<Self>) {
        DataContext::link(linker);
        Draw3dContext::link(linker);
    }
}
