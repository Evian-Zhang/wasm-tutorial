(module
    (import "outer" "log_number" (func $log_number (param i32)))
    (import "outer" "instability" (global $instability i32))

    (func $peek0 (param i32) (result i32 i32)
        local.get 0
        local.get 0
    )

    (func (export "wasm_mul") (param $left i32) (param $right i32) (result i32)
        (i32.add
            (i32.mul (local.get $left) (local.get $right))
            (global.get $instability)
        )
        call $peek0
        call $log_number
    )
)
