use std::{cell::RefCell, rc::Rc};

use nethercade_core::Rom;

mod audio_context;
pub use audio_context::PushedAudio;
use audio_context::*;

mod data;
use data::*;

mod draw_3d;
pub use draw_3d::DrawContextState;
use draw_3d::*;

mod input_context;
use input_context::InputContext;

use wasmtime::Linker;

use crate::graphics::VirtualGpu;

pub struct WasmContexts {
    pub data: DataContext,
    pub draw_3d: Draw3dContext,
    pub input: InputContext,
    pub audio: AudioContext,
}

impl WasmContexts {
    pub fn new(rom: &Rom, vgpu: Rc<RefCell<VirtualGpu>>, num_player: usize) -> Self {
        Self {
            data: DataContext::new(rom),
            draw_3d: Draw3dContext::new(vgpu),
            input: InputContext::new(num_player),
            audio: AudioContext::new(),
        }
    }

    pub fn link(linker: &mut Linker<Self>) {
        DataContext::link(linker);
        Draw3dContext::link(linker);
        InputContext::link(linker);
        AudioContext::link(linker);
    }
}
