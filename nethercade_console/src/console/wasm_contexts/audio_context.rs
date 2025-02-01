use wasmtime::{Caller, Linker};

use super::WasmContexts;

pub struct AudioContext {
    pub buffer_ptr: usize,
    pub buffer_len: usize,
    pub channel_count: usize,
}

impl AudioContext {
    pub fn new() -> Self {
        Self {
            buffer_ptr: 0,
            buffer_len: 0,
            channel_count: 0,
        }
    }

    pub fn link(linker: &mut Linker<WasmContexts>) {
        linker
            .func_wrap("env", "set_audio_buffer", set_audio_buffer)
            .unwrap();
    }
}

fn set_audio_buffer(
    mut caller: Caller<WasmContexts>,
    buffer_ptr: i32,
    buffer_len: i32,
    channel_count: i32,
) {
    let channel_count = channel_count.clamp(0, 2) as usize;
    let store = caller.data_mut();
    let audio = &mut store.audio;
    audio.buffer_ptr = buffer_ptr as usize;
    audio.buffer_len = buffer_len as usize;
    audio.channel_count = channel_count;
}
