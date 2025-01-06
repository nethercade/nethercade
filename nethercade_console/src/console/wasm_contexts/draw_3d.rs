use nethercade_core::Rom;
use wasmtime::{Caller, Linker};

use super::WasmContexts;

pub struct Draw3dContext {}

impl Draw3dContext {
    pub fn new(rom: &Rom) -> Self {
        Self {}
    }

    pub fn link(linker: &mut Linker<WasmContexts>) {
        linker
            .func_wrap(
                "env",
                "draw_tri_list",
                |mut caller: Caller<WasmContexts>, a: i32, b: i32, c: i32| {
                    caller.data_mut().draw_3d.draw_tri_list(a, b, c)
                },
            )
            .unwrap();
    }

    fn draw_tri_list(&mut self, data_ptr: i32, len: i32, pipeline: i32) {
        //TODO Write This
    }
}
