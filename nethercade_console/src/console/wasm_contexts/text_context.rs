use wasmtime::{Caller, Linker};

use super::WasmContexts;

pub struct TextContext;

impl TextContext {
    pub fn link(linker: &mut Linker<WasmContexts>) {
        linker.func_wrap("env", "console_log", console_log).unwrap();
        linker
            .func_wrap("env", "console_log_utf16", console_log_utf16)
            .unwrap();
    }
}

fn console_log(
    mut caller: Caller<WasmContexts>,
    text_ptr: i32,
    len: i32,
) -> Result<(), wasmtime::Error> {
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();

    let data = match mem
        .data(&caller)
        .get(text_ptr as u32 as usize..)
        .and_then(|arr| arr.get(..len as u32 as usize))
    {
        Some(data) => data,
        None => return Err(wasmtime::Error::msg("invalid data")),
    };

    let text = match std::str::from_utf8(data) {
        Ok(text) => text,
        Err(_) => return Err(wasmtime::Error::msg("string is not valid utf-8")),
    };

    println!("{text}");
    Ok(())
}

fn console_log_utf16(
    mut caller: Caller<WasmContexts>,
    text_ptr: i32,
    len: i32,
) -> Result<(), wasmtime::Error> {
    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();

    let data = match mem
        .data(&caller)
        .get(text_ptr as u32 as usize..)
        .and_then(|arr| arr.get(..len as u32 as usize))
    {
        Some(data) => data,
        None => return Err(wasmtime::Error::msg("invalid data")),
    };

    let text = match std::str::from_utf8(data) {
        Ok(text) => text,
        Err(_) => return Err(wasmtime::Error::msg("string is not valid utf-8")),
    };

    println!("{text}");
    Ok(())
}
