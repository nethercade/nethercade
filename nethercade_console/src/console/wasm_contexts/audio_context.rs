use bytemuck::cast_slice;
use wasmtime::{Caller, Linker};

use super::WasmContexts;

pub struct AudioContext {
    pub pushed_audio: Vec<PushedAudio>,
}

pub struct PushedAudio {
    pub channels: u16,
    pub data: Box<[f32]>,
    pub sample_rate: u32,
}

impl AudioContext {
    pub fn new() -> Self {
        Self {
            pushed_audio: Vec::new(),
        }
    }

    pub fn link(linker: &mut Linker<WasmContexts>) {
        linker.func_wrap("env", "push_audio", push_audio).unwrap();
    }

    fn push_audio(&mut self, channels: u16, data: &[f32], sample_rate: u32) {
        self.pushed_audio.push(PushedAudio {
            channels,
            data: data.into(),
            sample_rate,
        })
    }
}

fn push_audio(
    mut caller: Caller<WasmContexts>,
    buffer_ptr: i32,
    buffer_len: i32,
    channel_count: i32,
    sample_rate: i32,
) {
    let channel_count = channel_count.clamp(0, 2) as usize;
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
    let (data, store) = mem.data_and_store_mut(&mut caller);
    let data: &[f32] = cast_slice(&data[buffer_ptr as usize..]);
    store.audio.push_audio(
        channel_count as u16,
        &data[..buffer_len as usize],
        sample_rate as u32,
    );
}
