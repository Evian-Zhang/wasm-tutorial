# WASM的使用

在我们使用高级语言生成了WASM之后，毫无疑问接下来就是对WASM的使用。本章将主要讲述WASM的基本使用方法，而一些高级的使用方法在后续章节里会陆续介绍。

总的来说，在我们通过WASM引擎使用WASM程序的过程中，WASM标准规范了三个语义阶段：

* 解码（Decoding）

  通常来说，WASM是以二进制格式分发的。因此，在使用WASM时，第一步就是将二进制格式的WASM解码成内存中的内部表示。
* 验证（Validation）

  WASM本身是一个有类型的编程语言，除此之外也有许多保证正确性的约束。在解码之后，执行之前，WASM引擎会对WASM程序的正确性作验证。
* 执行（Execution）

  在验证完正确性之后，WASM引擎会真正执行这个WASM程序。

## 在Web中使用

## 在通用程序中使用

除了在Web中使用，由于其安全性与通用型，WASM目前也越来越多地在通用程序中作为库被使用。目前最常用的两个辅助库是[wasmer](https://wasmer.io)和[wasmtime](https://wasmtime.dev)。

wasmer目前支持的编程语言包括Rust, C/C++, JavaScript, Go, Python, PHP, Ruby, OCaml和Swift等。wasmtime目前支持的编程语言包括Rust, C/C++, Python, C#, Go, Ruby等。这里以Rust语言为例，介绍如何分别通过这两个库，在Rust程序中使用WASM库提供的函数。对于其他通用编程语言，这两个工具的官网上都有详细的指导，并且过程和原理与Rust语言并没有太大的差别。

### 使用wasmer

在Rust中使用wasmer的方式，在Hello world一章中已经介绍了。也就是在依赖中声明[wasmer](https://crates.io/crates/wasmer)这个库之后，编写如下代码：

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

下面我们就解释一下这代码中一些主要的语句的含义。

#### 读入程序

首先，我们通过Rust标准库，将WASM程序的字节码读入了内存`wasm_bytes`中，此时`wasm_bytes`的类型是`Vec<u8>`。这是因为我们后续在使用`Module::new`解析WASM时，会要求参数满足`AsRef<[u8]>`，因此我们一般都是直接读入内存即可。

#### Store

随后，我们创建了一个默认的`Store`类型变量。Store实际上是一个通用的[WASM中的概念](https://webassembly.github.io/spec/core/exec/runtime.html#store)，但是放在Rust的代码里讲，就直观很多。

我们知道，Rust是一个非常强调所有权和生命周期的编程语言。在一个WASM程序运行的过程中，会有很多全局的状态。例如，WASM程序中的函数本身，其生命周期理应是全局的。用来托管这些全局状态的，就称为Store。

除此之外，在wasmer中，Store还负责管理引擎。所谓引擎，就是指我们如何将WASM的内部表示，转译成原生的机器指令。例如：

```rust, ignore
use wasmer::{Store, EngineBuilder};
use wasmer_compiler_llvm::LLVM;

let compiler = LLVM::new();
let mut store = Store::new(compiler);
```

上述代码就是创建了一个，通过LLVM引擎来将WASM的内部表示转译成原生指令的Store。

LLVM引擎生成的原生字节码，目前是优化程度最高的，适合用于生产环境。而`Store::default`默认使用的是用Rust原生开发的，LLVM的平替[cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift)。这个引擎在转译时间和优化程度之间达到平衡，适用于开发环境。

#### Module

在之后详细解释WASM代码的章节中我们会了解到，一个WASM程序就是一个module。因此，我们之前提到的「WASM的内部表示」，实际上就是这样一个`Module`类型的变量。可以看到，这个变量创建的过程中，需要`store`作为参数。这是因为需要其需要引擎提供加持，所以要从store中提取目前的引擎。

创建`Module`类型的变量，就完成了之前提到的WASM语义阶段中的「解码」和「验证」两个阶段。

#### Imports

这个变量在与WASM的交互过程中必不可少，但目前我们暂时不需要了解，之后的章节会介绍。

#### Instance

当我们创建`Instance`类型的变量时，就是真正执行WASM程序的过程。创建这个变量，就会初始化WASM程序，形成一个WASM实例。也就是说，我们WASM程序从现在开始正式进入执行阶段。

#### 调用WASM导出函数

接下来，我们使用了`get_typed_function`来获得我们在WASM程序中导出的"add"函数，然后使用`call`来调用这个函数。我们知道，Rust是强类型的语言，所以需要提供这个函数的类型信息。`TypedFunction<(u32, u32), u32>`就提示编译器，这个`add`函数接收两个`u32`类型的参数，返回一个`u32`类型的值。

这里值得指出，为什么我们在生成WASM的时候，使用的是`usize`，而使用WASM时，使用的是`u32`呢？这是因为，在生成WASM的时候，Rust程序的目标平台为`wasm32-unknown-unknown`，是32位平台，所以`usize`自动成为了`u32`；而在使用WASM的时候，我们是要编译生成原生的Rust程序，例如目标平台为`x86_64-unknown-linux-gnu`。因此，`usize`是64位，我们需要手动使用`u32`才能保证类型信息是正确的。
