use anyhow::Result;
use std::fs;
use wasmer::{Imports, Instance, Module, Store, TypedFunction};

fn main() -> Result<()> {
    let wasm_bytes = fs::read("./adder.wasm")?;
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let imports = Imports::default();
    let instance = Instance::new(&mut store, &module, &imports)?;
    let add: TypedFunction<(u32, u32), u32> =
        instance.exports.get_typed_function(&mut store, "add")?;
    let sum = add.call(&mut store, 1, 2)?;
    println!("Sum is {sum}");
    Ok(())
}
