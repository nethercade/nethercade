use nethercade_core::{FrameRate, Resolution, Rom};
use wasmtime::{Caller, Linker};

use crate::console::WasmContexts;

pub struct DataContext {
    resolution: Resolution,
    frame_rate: FrameRate,
}

impl DataContext {
    pub fn new(rom: &Rom) -> Self {
        Self {
            resolution: rom.resolution,
            frame_rate: rom.frame_rate,
        }
    }

    pub fn link(linker: &mut Linker<WasmContexts>) {
        linker
            .func_wrap("env", "width", |caller: Caller<WasmContexts>| {
                caller.data().data.width()
            })
            .unwrap();
        linker
            .func_wrap("env", "height", |caller: Caller<WasmContexts>| {
                caller.data().data.height()
            })
            .unwrap();
        linker
            .func_wrap("env", "fps", |caller: Caller<WasmContexts>| {
                caller.data().data.fps()
            })
            .unwrap();
        linker
            .func_wrap("env", "frame_time", |caller: Caller<WasmContexts>| {
                caller.data().data.frame_time()
            })
            .unwrap();
    }

    fn width(&self) -> i32 {
        self.resolution.dimensions().0 as i32
    }

    fn height(&self) -> i32 {
        self.resolution.dimensions().1 as i32
    }

    fn fps(&self) -> i32 {
        self.frame_rate.frames_per_second() as i32
    }

    fn frame_time(&self) -> f32 {
        self.frame_rate.frame_time()
    }
}
