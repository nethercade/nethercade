use std::{cell::RefCell, rc::Rc};

use nethercade_core::Rom;

mod data;
use data::*;

mod draw_3d;
use draw_3d::*;

use wasmtime::Linker;

use crate::graphics::VirtualGpu;

pub struct WasmContexts {
    data: DataContext,
    pub draw_3d: Draw3dContext,
}

impl WasmContexts {
    pub fn new(rom: &Rom, vgpu: Rc<RefCell<VirtualGpu>>) -> Self {
        Self {
            data: DataContext::new(rom),
            draw_3d: Draw3dContext::new(vgpu),
        }
    }

    pub fn link(linker: &mut Linker<Self>) {
        DataContext::link(linker);
        Draw3dContext::link(linker);
    }
}
