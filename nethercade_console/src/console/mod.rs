use nethercade_core::Rom;
use wasmtime::{Config, Engine, Instance, Linker, Module, Store};

mod wasm_contexts;
use wasm_contexts::WasmContexts;

pub struct Console {
    store: Store<WasmContexts>,
    instance: Instance,
}

impl Console {
    pub fn new(rom: &Rom) -> Self {
        let engine = Engine::new(&Config::default()).unwrap();
        let module = Module::from_binary(&engine, &rom.code).unwrap();

        let mut linker = Linker::new(&engine);
        WasmContexts::link(&mut linker);

        let mut store = Store::new(&engine, WasmContexts::new(rom));
        let instance = linker.instantiate(&mut store, &module).unwrap();

        Self { store, instance }
    }

    pub fn call_wasm_func(&mut self, fn_name: &str) {
        if let Ok(func) = self
            .instance
            .get_typed_func::<(), ()>(&mut self.store, fn_name)
        {
            func.call(&mut self.store, ()).unwrap();
        }
    }
}
