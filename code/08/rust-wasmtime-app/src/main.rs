use anyhow::Result;
use std::fs;
use wasmtime::*;

fn main() -> Result<()> {
    let wasm_bytes = fs::read("./common.wasm")?;
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, wasm_bytes)?;
    let log_number = Func::wrap(&mut store, |number: i32| {
        println!("In WASM, we got {number}");
    });
    let instability = Global::new(
        &mut store,
        GlobalType::new(ValType::I32, Mutability::Const),
        (-5i32).into(),
    )?;
    let instance = Instance::new(
        &mut store,
        &module,
        &[log_number.into(), instability.into()],
    )?;
    let wasm_mul = instance.get_typed_func::<(u32, u32), u32>(&mut store, "wasm_mul")?;
    println!("Calculating 5 x 8 with instability -5 ...");
    let prod = wasm_mul.call(&mut store, (5, 8))?;
    println!("From outside, we got {prod}");
    Ok(())
}
