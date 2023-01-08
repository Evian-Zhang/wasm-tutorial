# WASM 汇编入门教程

在互联网大潮下，前端越来越「卷」。为了追求前端的性能，用于替代JavaScript的WebAssembly（简称WASM）的概念被提出。经过几年的发展与沉淀，开发者们发现WASM不仅仅在前端Web领域是一枚银弹：我们既可以在浏览器里运行WASM，也可以用[wasmtime](https://wasmtime.dev)在任意环境下原生地执行WASM代码；在通用编程语言领域，Rust有提案建议[将过程宏以WASM的形式执行](https://internals.rust-lang.org/t/pre-rfc-procmacros-implemented-in-wasm/10860)，并且有了相应的工具[watt](https://github.com/dtolnay/watt)；在运维领域，Docker正在尝试[使用WASM代替传统容器](https://docs.docker.com/desktop/wasm/)。

因此，WASM不再仅仅是前端圈子里的掌上明珠，也逐渐走入全栈所有领域开发者的视野。所以，我决定写一系列文章《WASM汇编入门》，面向所有领域的开发者，介绍WASM入门。

## 什么是WASM

那究竟什么是WASM呢？粗略来说，WASM就是一种新的指令集格式（类似于AMD64，AArch64等），也就是一种「字节码」。

从开发者的角度来说，WASM格式可以由后端常见的编程语言编译得到，如C、C++、Rust、C#、Go、Swift等（具体可见[Compile a WebAssembly module from...](https://webassembly.org/getting-started/developers-guide/)）。因此，后端的开发者可以用自己趁手的语言来开发WASM程序。

从使用者的角度来说，WASM程序可以像JavaScript程序一样，运行在Web浏览器中（目前火狐、Chrome、Safari和Edge均支持执行WASM程序，具体可参考[Roadmap](https://webassembly.org/roadmap/)），也可以通过wasmtime等方式，在任意环境下原生执行。因此，WASM程序具有很高的通用性。

## 为什么要使用WASM

### 前端

对于前端的开发者来说，原因非常简单：JavaScript就是依托答辩。

从编程语言的角度，对JavaScript的讨伐不绝于耳。尽管ES6等新标准力挽狂澜，给JavaScript增加了许多更利于开发者的特性，TypeScript的出现也让开发者维护项目更加方便，但是由于浏览器端天生需要非常严格的向前兼容性，许多因为历史原因而造成的失误无法弥补。因此，WASM的出现可以让前端开发者在开发WASM模块时，不再受JavaScript的折磨，可以选择更新更好更顺手的编程语言，维护更好的心情。

从性能的角度来看，尽管Google的V8引擎如前端的一针强心剂，将JavaScript代码的性能推向了原生级别，但是因为JavaScript语言本身的动态特性，性能还是会差一些。而WASM则是底层二进制程序格式，经过不同编程语言的优化，可以达到非常高的执行效率。Chrome的V8引擎（[WebAssembly compilation pipeline](https://v8.dev/docs/wasm-compilation-pipeline)）、火狐的SpiderMonkey引擎（[BaldrMonkey](https://spidermonkey.dev/docs/index.html#baldrmonkey)）、Safari的Webkit（[Assembling WebAssembly](https://webkit.org/blog/7691/webassembly/)）都支持WASM。
