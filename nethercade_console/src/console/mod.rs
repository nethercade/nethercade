use std::{cell::RefCell, rc::Rc, sync::Arc};

use eframe::wgpu;
use ggrs::{GgrsRequest, InputStatus};
use nethercade_core::{Resolution, Rom};
use network::{NetworkInputState, WasmConsoleState};
use wasmtime::{Config, Engine, Instance, Linker, Module, Store};

mod wasm_contexts;
use wasm_contexts::{DrawContextState, PushedAudio, WasmContexts};

mod input;
pub use input::{LocalInputManager, LocalPlayerId, MouseEventCollector};

mod network;
pub mod network_session;

pub mod gui;

use crate::{audio::AudioUnit, graphics::VirtualGpu};

pub struct GameInstance {
    pub store: Store<WasmContexts>,
    instance: Instance,
    pub rom: Rom,
    pub this_frame_audio: Vec<PushedAudio>,
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

    pub fn init(&mut self) {
        self.store.data_mut().draw_3d.state = DrawContextState::Init;
        self.call_wasm_func("init");
        self.store.data_mut().draw_3d.state = DrawContextState::Invalid;
    }

    pub fn update(&mut self) {
        self.call_wasm_func("update");
    }

    pub fn render(&mut self) {
        {
            let ctx = &mut self.store.data_mut().draw_3d;
            ctx.vrp.reset();
            // ctx.push_model_matrix(Mat4::IDENTITY);
            // TODO: push View matrix
            // TODO: Push Proj Matrix
            // ctx.set_texture(0);
            ctx.state = DrawContextState::Draw;
        }

        self.call_wasm_func("render");

        {
            let ctx = &mut self.store.data_mut().draw_3d;
            ctx.state = DrawContextState::Invalid;
            ctx.render();
        }
    }

    pub fn handle_requests(&mut self, requests: Vec<GgrsRequest<Self>>) {
        for request in requests {
            match request {
                GgrsRequest::LoadGameState { cell, .. } => {
                    let state = cell.data().expect("Failed to load game state");
                    self.load_save_state(&state);
                }
                GgrsRequest::SaveGameState { cell, frame } => {
                    let state = self.generate_save_state();
                    cell.save(frame, Some(state), None);
                }
                GgrsRequest::AdvanceFrame { inputs } => self.advance_frame(inputs),
            }
        }
    }

    fn generate_save_state(&mut self) -> WasmConsoleState {
        let previous_buttons = self
            .store
            .data()
            .input
            .input_entries
            .iter()
            .map(|input| input.previous)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let mem = self.instance.get_memory(&mut self.store, "memory").unwrap();

        let memory = mem.data(&mut self.store).to_vec();

        WasmConsoleState {
            previous_buttons,
            memory,
        }
    }

    pub fn load_save_state(&mut self, state: &WasmConsoleState) {
        let WasmConsoleState {
            previous_buttons,
            memory,
        } = state;

        previous_buttons
            .iter()
            .enumerate()
            .for_each(|(index, prev)| {
                self.store.data_mut().input.input_entries[index].previous = *prev;
            });

        self.instance
            .get_memory(&mut self.store, "memory")
            .unwrap()
            .data_mut(&mut self.store)
            .copy_from_slice(&memory);
    }

    fn advance_frame(&mut self, inputs: Vec<(NetworkInputState, InputStatus)>) {
        // Pre Update Input
        self.store
            .data_mut()
            .input
            .input_entries
            .iter_mut()
            .zip(inputs.iter())
            .for_each(|(current, new)| {
                current.current = new.0.input_state;
                current.current_mouse = new.0.mouse_state;
            });

        // Call WASM Update
        self.update();

        // Take only the "Most Recent" audio
        self.this_frame_audio.clear();
        self.this_frame_audio
            .extend(self.store.data_mut().audio.pushed_audio.drain(..));

        // Post Update Input
        self.store
            .data_mut()
            .input
            .input_entries
            .iter_mut()
            .for_each(|inputs| {
                inputs.previous = inputs.current.buttons;
                inputs.previous_mouse = inputs.current_mouse;
            });
    }
}

pub struct Console {
    pub game: Option<GameInstance>,
    pub vgpu: Rc<RefCell<VirtualGpu>>,
    pub audio: AudioUnit,
}

impl Console {
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
            audio: AudioUnit::new(),
        }
    }

    pub fn load_rom(rom: Rom, vgpu: Rc<RefCell<VirtualGpu>>, num_players: usize) -> GameInstance {
        let engine = Engine::new(&Config::default()).unwrap();
        let module = Module::from_binary(&engine, &rom.code).unwrap();

        let mut linker = Linker::new(&engine);
        WasmContexts::link(&mut linker);

        let mut store = Store::new(&engine, WasmContexts::new(&rom, vgpu.clone(), num_players));
        let instance = linker.instantiate(&mut store, &module).unwrap();

        vgpu.borrow_mut().resize(rom.resolution);

        let mut game_instance = GameInstance {
            store,
            instance,
            rom,
            this_frame_audio: Vec::new(),
        };

        game_instance.init();

        game_instance
    }
}
