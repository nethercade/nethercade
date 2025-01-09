use std::{cell::RefCell, rc::Rc, sync::Arc};

use eframe::wgpu;
use glam::Mat4;
use nethercade_core::{Resolution, Rom};
use wasmtime::{Config, Engine, Instance, Linker, Module, Store};

mod wasm_contexts;
use wasm_contexts::WasmContexts;

use crate::graphics::{frame_buffer::FrameBuffer, VirtualGpu};

pub struct GameInstance {
    store: Store<WasmContexts>,
    instance: Instance,
    pub rom: Rom,
}

impl GameInstance {
    fn call_wasm_func(&mut self, fn_name: &str) {
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
        let ctx = &mut self.store.data_mut().draw_3d;
        ctx.vrp.reset();
        ctx.push_matrix(Mat4::IDENTITY);
        ctx.set_texture(0);

        self.call_wasm_func("draw");
        self.store.data_mut().draw_3d.render();
    }
}

pub struct Console {
    pub game: Option<GameInstance>,
    pub vgpu: Rc<RefCell<VirtualGpu>>,
}

impl Console {
    pub fn get_frame_buffer(&self) -> Arc<FrameBuffer> {
        self.vgpu.borrow().frame_buffer.clone()
    }

    pub fn new(
        device: &Arc<wgpu::Device>,
        queue: &Arc<wgpu::Queue>,
        format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            vgpu: Rc::new(RefCell::new(VirtualGpu::new(
                Resolution::default(),
                device,
                queue,
                format,
            ))),
            game: None,
        }
    }

    pub fn load_rom(&mut self, rom: Rom, vgpu: Rc<RefCell<VirtualGpu>>) {
        let engine = Engine::new(&Config::default()).unwrap();
        let module = Module::from_binary(&engine, &rom.code).unwrap();

        let mut linker = Linker::new(&engine);
        WasmContexts::link(&mut linker);

        let mut store = Store::new(&engine, WasmContexts::new(&rom, vgpu));
        let instance = linker.instantiate(&mut store, &module).unwrap();

        self.vgpu.borrow_mut().resize(rom.resolution);

        self.game = Some(GameInstance {
            store,
            instance,
            rom,
        });
    }
}
