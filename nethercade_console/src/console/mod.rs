use std::sync::Arc;

use eframe::wgpu;
use glam::Mat4;
use nethercade_core::Rom;
use wasmtime::{Config, Engine, Instance, Linker, Module, Store};

mod wasm_contexts;
use wasm_contexts::WasmContexts;

use crate::graphics::frame_buffer::FrameBuffer;

pub struct Console {
    store: Store<WasmContexts>,
    instance: Instance,
    pub rom: Rom,
}

impl Console {
    pub fn get_frame_buffer(&self) -> Arc<FrameBuffer> {
        self.store.data().draw_3d.vgpu.frame_buffer.clone()
    }

    pub fn new(
        rom: Rom,
        device: &Arc<wgpu::Device>,
        queue: &Arc<wgpu::Queue>,
        format: wgpu::TextureFormat,
    ) -> Self {
        let engine = Engine::new(&Config::default()).unwrap();
        let module = Module::from_binary(&engine, &rom.code).unwrap();

        let mut linker = Linker::new(&engine);
        WasmContexts::link(&mut linker);

        let mut store = Store::new(&engine, WasmContexts::new(&rom, device, queue, format));
        let instance = linker.instantiate(&mut store, &module).unwrap();

        Self {
            store,
            instance,
            rom,
        }
    }

    pub fn call_wasm_func(&mut self, fn_name: &str) {
        if let Ok(func) = self
            .instance
            .get_typed_func::<(), ()>(&mut self.store, fn_name)
        {
            func.call(&mut self.store, ()).unwrap();
        }
    }

    pub fn update(&mut self) {
        self.call_wasm_func("update");
    }

    pub fn draw(&mut self) {
        {
            let vgpu = &mut self.store.data_mut().draw_3d.vgpu;
            vgpu.push_matrix(Mat4::IDENTITY);
            vgpu.set_texture(0);
        }

        self.call_wasm_func("draw");
        self.store.data_mut().draw_3d.vgpu.render();
    }
}
