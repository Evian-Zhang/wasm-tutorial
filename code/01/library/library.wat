(module ;; Define a module
    ;; Define a function with name `add`, two parameters of type i32, and returns i32
    (func $add (param $left i32) (param $right i32) (result i32)
        local.get $right ;; Push parameter `right` to stack
        local.get $left ;; Push parameter `left` to stack
        i32.add ;; Consume two values at stack top, and push the sum to stack
    )
    (export "add" (func $add)) ;; Export this function with symbol "add"
)
