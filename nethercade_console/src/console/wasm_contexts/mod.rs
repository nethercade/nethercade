use nethercade_core::Rom;

mod data;
use data::*;

mod draw_3d;
use draw_3d::*;

use wasmtime::Linker;

pub struct WasmContexts {
    data: DataContext,
    draw_3d: Draw3dContext,
}

impl WasmContexts {
    pub fn new(rom: &Rom) -> Self {
        Self {
            data: DataContext::new(rom),
            draw_3d: Draw3dContext::new(rom),
        }
    }

    pub fn link(linker: &mut Linker<Self>) {
        DataContext::link(linker);
        Draw3dContext::link(linker);
    }
}
