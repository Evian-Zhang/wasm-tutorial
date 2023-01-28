use anyhow::Result;
use std::fs;
use wasmer::{imports, Function, Global, Instance, Module, Store, TypedFunction, Value};

fn main() -> Result<()> {
    let wasm_bytes = fs::read("./common.wasm")?;
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let imports = imports! {
        "outer" => {
            "log_number" =>
                Function::new_typed(
                    &mut store,
                    |number: i32| println!("In WASM, we got {number}")
                ),
            "instability" => Global::new(&mut store, Value::I32(-5)),
        }
    };
    let instance = Instance::new(&mut store, &module, &imports)?;
    let wasm_mul: TypedFunction<(u32, u32), u32> = instance
        .exports
        .get_typed_function(&mut store, "wasm_mul")?;
    println!("Calculating 5 x 8 with instability -5 ...");
    let prod = wasm_mul.call(&mut store, 5, 8)?;
    println!("From outside, we got {prod}");
    Ok(())
}
