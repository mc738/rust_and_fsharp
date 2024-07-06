use std::env;
use std::error::Error;
use wasi_common::sync::WasiCtxBuilder;
use wasmtime::{Engine, Module, Store, Linker};

fn run_module(path: &String) -> Result<(), Box<dyn Error>> {
    // Set up the engine.
    // https://docs.wasmtime.dev/lang-rust.html
    let engine = Engine::default();

    let module = Module::from_file(&engine, path)?;

    let mut linker = Linker::new(&engine);

    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    linker.module(&mut store, "", &module)?;

    linker.get_default(&mut store, "")?.typed::<(), ()>(&store)?.call(&mut store, ())?;

    Ok (())
}

fn main() -> Result<(), Box<dyn Error>>  {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        run_module(&args[1])
    }
    else {
        Err(Box::from("Missing .wasm/.wat path argument"))
    }
}
