(module
    (func $add (param $left i32) (param $right i32) (result i32)
        local.get $left
        local.get $right
        i32.add
    )
    (func $sub (param $left i32) (param $right i32) (result i32)
        local.get $left
        local.get $right
        i32.sub
    )
    (export "sub" (func $sub))
)
