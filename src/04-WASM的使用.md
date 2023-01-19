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

WASM程序目前最多的用途，就是在Web中使用了。本节就主要介绍几种在Web中使用WASM的常见方式。

### 基础方式

在Hello world一章中，我们展示了在Web中使用WASM程序的基础方式：

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

在这里，我们调用了内置的WebAssembly模块的[`instantiateStreaming`](https://developer.mozilla.org/en-US/docs/WebAssembly/JavaScript_interface/instantiateStreaming)函数。这个函数是目前最主要使用的引入WASM程序的接口。同时也从这个的使用方式看出来，当我们需要引入WASM模块时，必须要**手动请求**这个资源，例如使用`fetch`等函数；此外，这个函数是异步的，说明目前引入WASM模块时，也都是异步的请求。

当我们查看这个函数的API文档时，会发现其接收第二个参数`importObject`，这个暂时不细讲，之后讲到WASM函数的导入与导出时再详细解释。

此外，这个函数实际上做了之前提到的WASM语义阶段中的「解码」与「验证」两个步骤（严格来说，也做了「执行」步骤中的「初始化」一步，不过不重要）。

在使用`instantiateStreaming`函数引入WASM之后，我们可以通过`obj.instance.exports.add`来访问我们之前导出的`add`函数。

### ES Module形式导入

在JavaScript来到新时代之后，ES Module让JavaScript多文件组织变得异常清晰与简便。那我们多么希望能够这样使用WASM模块：

```javascript
import { add } from "./adder.wasm";

console.log(add(1, 2));
```

如果WASM模块也能以ES Module的形式来导入，这该多好啊！

事实上，这个功能目前还是一个提案[ES Module Integration Proposal for WebAssembly](https://github.com/WebAssembly/esm-integration)，没有被广泛地实现。目前，实现这个功能还有一些阻碍，例如，之前我们提到，对WASM的引入都是异步的，因此如果要全局直接`import`，势必要Top-level Await，而这个也没有很好地实现。

但是，尽管目前没有普遍实现，一个好消息是：[Webpack支持以ES Module的形式导入WASM程序](https://webpack.js.org/configuration/module/#ruletype)！具体的代码可以参考本仓库的[web-webpack](https://github.com/Evian-Zhang/wasm-tutorial/tree/main/code/04/web-webpack)目录，其中的代码参考了[ballercat/minimal-webpack5-wasm-demo](https://github.com/ballercat/minimal-webpack5-wasm-demo)。

简单而言，我们只需要在`webpack.config.js`中加入下面一段代码：

```javascript
module.exports = {
  // ...
  experiments: {
    asyncWebAssembly: true,
  },
  // ...
};
```

就可以以ES Module的方式引入WASM程序啦！

## 在通用程序中使用

除了在Web中使用，由于其安全性与通用型，WASM目前也越来越多地在通用程序中作为库被使用。目前最常用的两个辅助库是[wasmer](https://wasmer.io)和[wasmtime](https://wasmtime.dev)。

wasmer目前支持的编程语言包括Rust, C/C++, JavaScript, Go, Python, PHP, Ruby, OCaml和Swift等。wasmtime目前支持的编程语言包括Rust, C/C++, Python, C#, Go, Ruby等。这里以Rust语言为例，介绍如何分别通过这两个库，在Rust程序中使用WASM库提供的函数。对于其他通用编程语言，这两个工具的官网上都有详细的指导，并且过程和原理与Rust语言并没有太大的差别。并且对于C/C++来说，WASM官方也正在推进统一的[WebAssembly C and C++ API](https://github.com/webassembly/wasm-c-api)，不过目前还不成熟。

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
    let add: TypedFunction<(u32, u32), u32> =
        instance.exports.get_typed_function(&mut store, "add")?;
    let sum = add.call(&mut store, 1, 2)?;
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

### 使用wasmtime

wasmtime与wasmer类似，也是一个可以用作WASM引擎的库。我们声明[wasmtime](https://crates.io/crates/wasmtime)之后，编写如下代码：

```rust, ignore
use anyhow::Result;
use std::fs;
use wasmtime::*;

fn main() -> Result<()> {
    let wasm_bytes = fs::read("./adder.wasm")?;
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, wasm_bytes)?;
    let instance = Instance::new(&mut store, &module, &[])?;
    let add = instance.get_typed_func::<(u32, u32), u32>(&mut store, "add")?;
    let sum = add.call(&mut store, (1, 2))?;
    println!("Sum is {sum}");
    Ok(())
}
```

上述代码与之前wasmer的代码做了一样的事。

关于上述代码，大部分是与wasmer中的类似，其中，与wasmer的代码最大的不同，是`Store::new`创建时的第二个参数与`Instance::new`创建时的第三个参数。这两个参数实际上对应的是wasmer中的`Imports`，因此在这里暂时不作细讲，之后介绍WASM函数的导入与导出时再详细介绍。

此外，目前wasmtime主要使用的是cranelift来将WASM内部表示生成为原生机器码，而不像wasmer一样提供LLVM等方式。

### 总结

总结而言，在上述通用程序的使用中，我们可以发现，对WASM的解码、验证与执行，都是**运行时**执行的，而非编译时或链接时执行的。这与我们传统的使用静态链接库的方式不一样，更类似于我们使用动态链接库。这是因为，我们将WASM作为库在通用程序中使用的时候，往往是考虑其较高的安全性与平台的支持性，因此大部分情况下，这些WASM程序都是作为插件系统来使用。而插件系统的特点，就是在运行时加载、执行，因此目前相关的API设计才是如此。
