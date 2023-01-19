use anyhow::Result;
use std::fs;
use wasmtime::*;

fn main() -> Result<()> {
    let wasm_bytes = fs::read("./adder.wasm")?;
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, wasm_bytes)?;
    let instance = Instance::new(&mut store, &module, &[])?;
    let add = instance.get_typed_func::<(u32, u32), u32>(&mut store, "add")?;
    let sum = add.call(&mut store, (1, 2))?;
    println!("Sum is {sum}");
    Ok(())
}
