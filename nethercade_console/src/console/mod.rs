use wasmtime::{Config, Engine, Instance, Module, Store};

pub struct Console {
    store: Store<WasmState>,
    instance: Instance,
}

pub struct WasmState {}

impl Console {
    pub fn new(code: &[u8]) -> Self {
        let engine = Engine::new(&Config::default()).unwrap();
        let module = Module::from_binary(&engine, code).unwrap();

        let mut store = Store::new(&engine, WasmState {});
        // TODO: Populate this with functions
        let imports = [];
        let instance = Instance::new(&mut store, &module, &imports).unwrap();

        Self { store, instance }
    }

    pub fn call_wasm_func(&mut self, fn_name: &str) {
        if let Ok(func) = self
            .instance
            .get_typed_func::<(), ()>(&mut self.store, fn_name)
        {
            func.call(&mut self.store, ());
        }
    }
}
