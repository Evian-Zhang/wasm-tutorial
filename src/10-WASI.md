# WASI

之前我们提到，要想解决WASM程序与外界交互的问题，方法之一就是使用导入与导出。有没有方法之二呢？答案是「YES AND NO」。在这章中我们要讨论的[WASI](https://wasi.dev)（The WebAssembly System Interface），可以看做方法之二，而它实际上也是在导入与导出基础之上的。

WASI的想法非常简单，我们在WASM中使用`import`对外界请求导入函数时，有些功能往往被非常多WASM程序请求，例如读文件、写文件等等。那WASI就是把这些功能抽象出来，既可以理解成操作系统提供的系统调用接口，也可以理解成libc的统一接口，总之是一个抽象的接口。

WASI定义的是一个接口标准，而各个嵌入环境的实现可以选择性地实现接口。例如，Web环境肯定是不允许直接读写文件的，因此Web环境可以选择不实现对文件读写的接口。但总之，通过WASI接口，我们的WASM程序就可以实现更多的可移植性了。

值得一提的是，WASI目前还是一个很新的技术，因此目前的标准、规定也不是稳定的。所以在这一章中，我们主要讨论的是WASI的基本原理，而不会详细阐述WASI的API。

## 使用WASI的WASM程序

首先我们看看使用WASI的WASM程序长什么样：

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

这熟悉的感觉，这不就是我们最开始Hello world那一章的例子嘛！在经历了这么多之后，我们再来看这段代码，是不是清晰了很多呢。

没错，这就是一个简单的输出"Hello world\n"的程序嘛。唯一值得注意的，就是`import`中的"wasi_unstable"模块，以及将`$main`函数导出为`_start`这两点。

我们之前在讲导入导出时提过，`import`后的第一个字符串是模块名，而我们使用Rust/JavaScript向其导入函数时，并没有什么特殊的操作，只是把这个模块名当作一个key来用。而在这里，"wasi_unstable"就是指WASI这个模块名。当我们的模块名是这个（或者"wasi_snapshot_preview1"，**目前**并没有什么区别）时，就代表我们想引入的是WASI规定的接口，这里就是"fd_write`这个函数。

那么，将`$main`函数导出为`_start`这个符号又是为什么呢？这目前遵循的是[C ABI的入口点标准](https://github.com/WebAssembly/tool-conventions/blob/main/BasicCABI.md#program-entrypoint)以及[WASI Application ABI](https://github.com/WebAssembly/WASI/blob/main/legacy/application-abi.md#current-unstable-abi)，也就是说，当我们的程序作为独立的程序在操作系统中执行时，程序的入口点目前的符号就是`_start`。

## WASI程序的使用

那么，我们该如何使用WASI程序呢？首先值得指出，遵循WASI接口的WASM程序，它在二进制层面，和普通的WASM程序没有任何区别，它就是个WASM程序。因此，我们还是可以通过`wat2wasm`将文本格式转变为二进制格式。

### 作为独立程序使用

当我们编写的WASI程序是一个独立程序时（例如上面的hello world程序），在Hello world一章中我们提到，可以直接

```shell
wasmer run standalone.wasm
```

使用`wasmer`或者`wasmtime`直接运行。

### 作为库使用

从某种意义上来说，将WASI程序作为独立程序使用，就是一种将其作为库使用的特殊情形。因此，我们来讨论一下，将WASI程序作为库使用是怎样的。

我们之前提到，WASI本身只是一个接口，还需要嵌入环境的实现。因此，当我们执行WASI程序时，是需要给出其实现的：

* 当我们在Web上使用WASI程序时，需要使用[WASI polyfill](https://wasi.dev/polyfill/)
* 当我们使用wasmer引擎运行WASI程序时，需要使用[wasmer-wasi](https://crates.io/crates/wasmer-wasi)
* 当我们使用wasmtime引擎运行WASI程序时，需要使用[wasmtime-wasi](https://crates.io/crates/wasmtime-wasi)

目前来看，我们使用的逻辑就是：

各个环境实现WASI --> 在执行的时候，由引擎将实现导入 --> 执行WASI程序

## WASI程序的生成

当我们使用C/C++或Rust生成WASI程序时，如果还需要手动引入WASI头文件，然后手动调用WASI提供的接口，未免有些麻烦了。事实上，对于C/C++而言，我们有[wasi-libc](https://github.com/WebAssembly/wasi-libc)，而Rust的libc也有[wasi版本](https://rust-lang.github.io/libc/wasm32-wasi/doc/libc/index.html)。

简单来说，就是我们使用WASI定义的接口，实现了大部分libc的函数。那么我们基于libc写的C、Rust函数，就可以无缝生成wasi版本了。

## Component Model

不考虑我们手写WASM程序的情形，那么我们遵循WASI接口的WASM程序，从生成到使用，其步骤是

1. 各平台实现基于WASI接口的libc
2. 正常使用高级语言编写基于libc接口的程序
3. 编译器基于WASI接口的libc，生成遵循WASI接口的WASM程序
4. 针对不同的执行引擎，引入相应的WASI实现
5. 执行引擎将WASI实现导入WASM模块
6. 执行引擎执行WASM程序

我们会发现，在生成WASI程序的时候，看上去还不错，耦合性并不高，基于WASI接口的libc根据平台实现，不同编程语言的libc接口由编程语言实现，WASM程序的生成由编译器实现。但是到了执行WASI程序的时候，耦合性就上来了。每个执行引擎都有WASI的实现，但这似乎毫无必要，因为WASI的实现应该与执行引擎无关才对。

因此，目前WASM社区正在摸索一条基于「[Component Model](https://github.com/WebAssembly/component-model)」的道路。这个模型不仅适用于WASI，实际上也适用于大部分的导入导出情形。简单而言，就是说我们在执行WASM程序的时候，可以粗略地看做三部分：执行引擎、WASM程序、其他语言编写的导入函数。执行引擎通过将其他语言编写的导入函数导入到WASM程序之中，就可以执行WASM程序。这样的话，就能巧妙地解决我们上述提到的，WASI的实现耦合性较高的问题了。
