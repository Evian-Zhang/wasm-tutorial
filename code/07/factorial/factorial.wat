(module
    (func $factorial_recur (export "factorial_recur") (param $input i32) (result i32)
        (i32.le_u (local.get $input) (i32.const 1))
        if
            i32.const 1
            return
        end
        (i32.mul
            (local.get $input)
            (call $factorial_recur (i32.sub (local.get $input) (i32.const 1)))
        )
    )

    (func (export "factorial_iter_register") (param $input i32) (result i32)
        (local $prod i32)
        (local.set $prod (i32.const 1))
        loop $main_loop (result i32)
            (i32.le_u (local.get $input) (i32.const 1))
            if
                local.get $prod
                return
            end
            (local.set $prod
                (i32.mul (local.get $input) (local.get $prod))
            )
            (local.set $input (i32.sub (local.get $input) (i32.const 1)))
            br $main_loop
        end
    )

    (func $peek0 (param i32) (result i32 i32)
        local.get 0
        local.get 0
    )

    (func $peek1 (param i32 i32) (result i32 i32 i32)
        local.get 0
        local.get 1
        local.get 0
    )

    (func (export "factorial_iter_stack") (param $input i32) (result i32)
        i32.const 1
        local.get $input
        loop $main_loop (param i32 i32) (result i32)
            call $peek1
            call $peek1
            i32.mul
            call $peek1
            i32.const 1
            i32.sub
            call $peek0
            i32.const 1
            i32.gt_u
            br_if $main_loop
            call $peek1
            return
        end
    )
)
