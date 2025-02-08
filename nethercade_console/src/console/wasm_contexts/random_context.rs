use fastrand::Rng;
use wasmtime::{Caller, Linker};

use super::WasmContexts;

#[derive(Clone)]
pub struct RandomContext {
    shared_rng: Rng,
}

impl RandomContext {
    pub fn new(shared_seed: u64) -> Self {
        Self {
            shared_rng: Rng::with_seed(shared_seed),
        }
    }

    pub fn link(linker: &mut Linker<WasmContexts>) {
        linker.func_wrap("env", "set_seed", set_seed).unwrap();
        linker
            .func_wrap("env", "random_int_range", random_int_range)
            .unwrap();
        linker
            .func_wrap("env", "random_float", random_float)
            .unwrap();
        linker
            .func_wrap("env", "random_float_range", random_float_range)
            .unwrap();
    }

    fn set_seed(&mut self, seed: i64) {
        self.shared_rng.seed(seed as u64);
    }

    fn random_int_range(&mut self, min: i32, max: i32) -> i32 {
        self.shared_rng.i32(min..max)
    }

    fn random_float(&mut self) -> f32 {
        self.shared_rng.f32()
    }

    fn random_float_range(&mut self, min: f32, max: f32) -> f32 {
        let range = max - min;
        let scale = self.shared_rng.f32() * max;
        (scale * range) + min
    }
}

fn set_seed(mut caller: Caller<WasmContexts>, seed: i64) {
    caller.data_mut().random.set_seed(seed);
}

fn random_int_range(mut caller: Caller<WasmContexts>, min: i32, max: i32) -> i32 {
    caller.data_mut().random.random_int_range(min, max)
}

fn random_float(mut caller: Caller<WasmContexts>) -> f32 {
    caller.data_mut().random.random_float()
}

fn random_float_range(mut caller: Caller<WasmContexts>, min: f32, max: f32) -> f32 {
    caller.data_mut().random.random_float_range(min, max)
}
