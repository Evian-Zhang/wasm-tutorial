(module
    (memory (export "memory") 1)
    (func (export "transform") (param $index i32) (param $length i32)
        (local $ch i32)
        loop $main_loop
            (i32.le_u (local.get $length) (i32.const 0))
            if
                return
            end
            (i32.store8
                (local.get $index)
                (i32.add
                    (i32.load8_u (local.get $index))
                    (i32.const 1)
                )
            )
            (local.set $index (i32.add (local.get $index) (i32.const 1)))
            (local.set $length (i32.sub (local.get $length) (i32.const 1)))
            br $main_loop
        end
    )
)
