# Hello world

正式学习WASM汇编的第一步，是先对WASM有一个初步的认识。因此，正如在学习别的编程语言的过程中，一个Hello world是必不可少的，本章也将展示WASM的一些基本程序。

正如我们之前提到的，WASM主要有两个用途：一个是作为一个库，提供一些函数，给Web上的JavaScript调用或者给通用的程序调用；另一个是作为一个独立的程序在后端使用wasmer或者wasmtime等工具来直接执行。相应地，我们的程序，也有两个版本：库版本和独立的程序版本。

## 作为库

我们创建一个文本文件`library.wat`，其内容为

```wasm
(module ;; Define a module
    ;; Define a function with name `add`, two parameters of type i32, and returns i32
    (func $add (param $left i32) (param $right i32) (result i32)
        local.get $left ;; Push parameter `left` to stack
        local.get $right ;; Push parameter `right` to stack
        i32.add ;; Consume two values at stack top, and push the sum to stack
    )
    (export "add" (func $add)) ;; Export this function with symbol "add"
)
```

这就是我们这一系列学习的核心，WASM汇编。看上去确实花里胡哨，搞JavaScript这种高级语言的看不懂，搞AMD64、AArch64这种汇编语言的看这也感觉奇形怪状。

第一眼看不懂不要紧，之后我们会详细解释每一行每一个语句的意思。这里我们可以通过注释，简单了解到，这实际上是定义了一个名字是`add`的函数，将两个32位有符号整数相加并返回他们的和。

接下来我们要做的，是将这个文本文件转变成二进制文件。这里我们需要使用[wabt](https://github.com/WebAssembly/wabt)工具链，其README里有编译或下载安装的方法（对于Linux和macOS用户，也可以[使用Homebrew下载](https://formulae.brew.sh/formula/wabt)）。当我们安装了wabt工具链之后，使用如下命令：

```shell
wat2wasm -o adder.wasm library.wat
```

这个命令将文本形式的`library.wat`翻译成了二进制形式的`adder.wasm`。

接下来，我们怎么使用这个WASM库呢？

### 在Web上使用

WASM模块目前最常用的场景，就是在Web上使用。因此，我们可以写一个基础的HTML：

```html
<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>WASM Test</title>
  </head>

  <body>
    <script>
      WebAssembly.instantiateStreaming(fetch("./adder.wasm"))
        .then(obj => {
            console.log(obj.instance.exports.add(1, 2));
        });
    </script>
  </body>
</html>
```

我们注意到，需要使用`fetch`来获取这个wasm文件，因此由于浏览器的同源策略，我们不能直接双击打开这个html来看效果。我们可以用一些简易的服务程序，例如[serve](https://github.com/vercel/serve)等，启动后访问相应的url，然后在控制台上，我们可以看到输出了`3`，说明正常运行了。

这里我们看到有很多不同的API。关于具体的如何在Web上与WASM交互，之后的章节会有更详细的说明。

### 在通用程序中使用

这里以Rust为例，我们看看如何在Rust程序中使用WASM模块提供的函数。

首先，我们需要引入[wasmer](https://crates.io/crates/wasmer)库（[wasmtime](https://crates.io/crates/wasmtime)等其他库也可以）。我们的Rust程序为：

```rust, ignore
use anyhow::Result;
use std::fs;
use wasmer::{Imports, Instance, Module, Store, TypedFunction};

fn main() -> Result<()> {
    let wasm_bytes = fs::read("./adder.wasm")?;
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let imports = Imports::default();
    let instance = Instance::new(&mut store, &module, &imports)?;
    let run_func: TypedFunction<(u32, u32), u32> =
        instance.exports.get_typed_function(&mut store, "add")?;
    let sum = run_func.call(&mut store, 1, 2)?;
    println!("Sum is {sum}");
    Ok(())
}
```

编译运行后，会输出"Sum is 3"。由此可见，Rust程序也可以使用WASM提供的函数。

关于Rust等通用程序如何与WASM模块交互，之后也会有章节进行具体说明。

## 作为独立程序

WASM作为库的使用看上去非常简单，我们需要写的WASM汇编代码也很少，不过实现的功能也相对简单，只是一个相加求和的功能。接下来，我们真真正正地写一个Hello world来看看！（以下代码出自[bytecodealliance/wastime](https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md#web-assembly-text-example)）

```wasm
(module
    ;; Import the required fd_write WASI function which will write the given io vectors to stdout
    ;; The function signature for fd_write is:
    ;; (File Descriptor, *iovs, iovs_len, nwritten) -> Returns number of bytes written
    (import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))

    (memory 1)
    (export "memory" (memory 0))

    ;; Write 'hello world\n' to memory at an offset of 8 bytes
    ;; Note the trailing newline which is required for the text to appear
    (data (i32.const 8) "hello world\n")

    (func $main (export "_start")
        ;; Creating a new io vector within linear memory
        (i32.store (i32.const 0) (i32.const 8))  ;; iov.iov_base - This is a pointer to the start of the 'hello world\n' string
        (i32.store (i32.const 4) (i32.const 12))  ;; iov.iov_len - The length of the 'hello world\n' string

        (call $fd_write
            (i32.const 1) ;; file_descriptor - 1 for stdout
            (i32.const 0) ;; *iovs - The pointer to the iov array, which is stored at memory location 0
            (i32.const 1) ;; iovs_len - We're printing 1 string stored in an iov - so one.
            (i32.const 20) ;; nwritten - A place in memory to store the number of bytes written
        )
        drop ;; Discard the number of bytes written from the top of the stack
    )
)
```

同样地，我们把这个程序翻译成二进制格式：

```shell
wat2wasm -o standalone.wasm standalone.wat
```

如果要执行这个WASM程序，我们需要安装一个WASM运行时，比较常见的包括[wasmer](https://wasmer.io)和[wasmtime](https://wasmtime.dev)。

我们以wasmer为例，使用

```shell
wasmer run standalone.wasm
```

我们可以看到，屏幕上输出了"hello world"，成功了！
