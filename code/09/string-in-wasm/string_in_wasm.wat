(module
    (import "outer" "memory" (memory 1))
    (import "outer" "log" (func $log (param i32 i32)))

    (data (i32.const 0) "关注希月萌奈喵")

    (func (export "output_inside_string")
        (call $log (i32.const 0) (i32.const 21))
    )
)
