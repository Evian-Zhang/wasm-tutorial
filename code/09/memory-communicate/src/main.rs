use anyhow::Result;
use std::fs;
use wasmer::{imports, Instance, Module, Store, TypedFunction};

fn main() -> Result<()> {
    let wasm_bytes = fs::read("./transformer.wasm")?;
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let imports = imports! {};
    let instance = Instance::new(&mut store, &module, &imports)?;
    let transform: TypedFunction<(u32, u32), ()> = instance
        .exports
        .get_typed_function(&mut store, "transform")?;
    let memory = instance.exports.get_memory("memory")?;

    // Prepare source
    let source: [u8; 6] = [1, 1, 4, 5, 1, 4];
    let memory_view = memory.view(&store);
    memory_view.write(0, &source)?;

    transform.call(&mut store, 0, 6)?;

    // Retrieve transformed source
    let memory_view = memory.view(&store);
    let mut transformed_source = [0; 6];
    memory_view.read(0, &mut transformed_source)?;

    println!("Transformed source is {transformed_source:?}");

    Ok(())
}
