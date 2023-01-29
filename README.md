# WASM 汇编入门教程

本系列教程的GitHub仓库为[Evian-Zhang/wasm-tutorial](https://github.com/Evian-Zhang/wasm-tutorial)，所有代码的源代码位于`code`目录下，推荐在[evian-zhang.github.io/wasm-tutorial](https://evian-zhang.github.io/wasm-tutorial)中阅读，获得最佳体验。如需PDF版本，可在对应网页的右上角点击「打印」即可。

在互联网大潮下，前端越来越「卷」。为了追求前端的性能，用于替代JavaScript的WebAssembly（简称WASM）的概念被提出。经过几年的发展与沉淀，开发者们发现WASM不仅仅在前端Web领域是一枚银弹：我们既可以在浏览器里运行WASM，也可以用[wasmer](https://wasmer.io)、[wasmtime](https://wasmtime.dev)等方式在任意环境下原生地执行WASM代码；在通用编程语言领域，Rust有提案建议[将过程宏以WASM的形式执行](https://internals.rust-lang.org/t/pre-rfc-procmacros-implemented-in-wasm/10860)，并且有了相应的工具[watt](https://github.com/dtolnay/watt)；在运维领域，Docker正在尝试[使用WASM代替传统容器](https://docs.docker.com/desktop/wasm/)。

因此，WASM不再仅仅是前端圈子里的掌上明珠，也逐渐走入全栈所有领域开发者的视野。所以，我决定写一系列文章《WASM汇编入门》，面向所有领域的开发者，介绍WASM入门。

## 本系列文章讲了什么

本系列文章，将从头开始，以较底层的WASM汇编角度，全方位介绍WASM程序的生成、使用，以及WASM中的概念、汇编语法。

本系列文章是我自己学习的一个记录，必然会有所错误、缺漏。还望大家不吝[提出issue](https://github.com/Evian-Zhang/wasm-tutorial/issues/new/choose)、[发起PR](https://github.com/Evian-Zhang/wasm-tutorial/compare)，相互指导，谢谢大家！

## 预备知识

本系列文章的预备知识包括：

* 高级编程语言

   * 了解JavaScript及Web开发相关知识
   * 了解Rust、C开发相关概念及知识
* 底层汇编

   * 了解LLVM相关概念（可参考我写的[LLVM IR入门指南](https://github.com/Evian-Zhang/llvm-ir-tutorial)）
   * 了解一门常见的汇编语言，如AMD64架构汇编（可参考我写的[macOS上的汇编入门](https://github.com/Evian-Zhang/Assembly-on-macOS)）或AArch64架构汇编（可参考我写的[在 Apple Silicon Mac 上入门汇编语言](https://github.com/Evian-Zhang/learn-assembly-on-Apple-Silicon-Mac)）

这些预备知识并不是要求读者掌握的炉火纯青，而是大部分常见的概念要了解，以及一些简单的代码要能够看懂。在绝大部分以概念为主的章节中，我基本上是不会涉及具体的高级语言或者汇编语言的代码的，只有在与代码强相关的章节中，我才会以例子的形式引入相应的代码。

## 环境

本文的所有代码的测试环境为

* 操作系统

   Ubuntu 22.04.1
* CPU

   Intel i9-12900K
* Clang

   Homebrew clang version 15.0.7
* emcc

   emcc (Emscripten gcc/clang-like replacement + linker emulating GNU ld) 3.1.5
* Rust

   rustc 1.67.0 (fc594f156 2023-01-24)
* wasmer

   wasmer 3.1.1
* wabt

   1.0.27

## 什么是WASM

那究竟什么是WASM呢？粗略来说，WASM是一种接近底层的「中间语言」。虽然它的名字里带有「Web」，但实际上它并不仅仅适用于Web。对于对编译器理论更熟悉的开发者来说，WASM与JVM字节码的用法非常相似，也类似于LLVM IR。通俗来讲，就是一种类似汇编语言的编程语言，可以由各种语言编译而来，可以在各种平台上执行。

从开发者的角度来说，WASM格式可以由后端常见的编程语言编译得到，如C、C++、Rust、C#、Go、Swift等（具体可见[Compile a WebAssembly module from...](https://webassembly.org/getting-started/developers-guide/)）。因此，后端的开发者可以用自己趁手的语言来开发WASM程序。

从使用者的角度来说，WASM程序可以像JavaScript程序一样，运行在Web浏览器中（目前火狐、Chrome、Safari和Edge均支持执行WASM程序，具体可参考[Roadmap](https://webassembly.org/roadmap/)），也可以通过wasmer、wasmtime等方式，在如Windows、macOS、Linux等原生环境下执行。因此，WASM程序具有很高的通用性。

## 为什么要使用WASM

### 兼容性

成熟的技术选型，往往考虑最多的是技术的「兼容性」。如果一个技术在开发上不兼容（需要单独招程序员）、在使用上不兼容（「本网站请使用IE打开」），那就是依托答辩。

从开发上来说，理论上，凡是以LLVM为后端的编程语言，都支持生成WASM程序。不说别的，主流的C/C++代码，均可生成WASM程序。

从使用上来说，要求能在各个操作系统原生执行已经是基本要求了。WASM不仅如此，能在所有主流浏览器上执行！这可了不得了，Java Applet、Flash都哭了。目前Chrome的V8引擎（[WebAssembly compilation pipeline](https://v8.dev/docs/wasm-compilation-pipeline)）、火狐的SpiderMonkey引擎（[BaldrMonkey](https://spidermonkey.dev/docs/index.html#baldrmonkey)）、Safari的Webkit（[Assembling WebAssembly](https://webkit.org/blog/7691/webassembly/)）都支持WASM。

### 前端积怨已久

在前端，从编程语言的角度，对JavaScript的讨伐不绝于耳。尽管ES6等新标准力挽狂澜，给JavaScript增加了许多更利于开发者的特性，TypeScript的出现也让开发者维护项目更加方便，但是由于浏览器端天生需要非常严格的向前兼容性，许多因为历史原因而造成的失误无法弥补。因此，WASM的出现可以让前端开发者在开发WASM模块时，不再受JavaScript的折磨，可以选择更新更好更顺手的编程语言，维护更好的心情。

### 性能优势

从性能的角度来看，尽管Google的V8引擎如前端的一针强心剂，将JavaScript代码的性能推向了原生级别，但是因为JavaScript语言本身的动态特性，性能还是会差一些。尽管JavaScript本身的特性更加适用于Web上针对DOM的灵活操作、请求响应JSON的解析等功能，但是随着前端负责的功能模块越来越广泛，对于计算密集型的操作，如密码学加解密、图像处理等操作，JavaScript的动态特性会减弱优化程度。而WASM则是底层二进制程序格式，由不同的编程语言编译而来。因此，对代码的优化就不再依赖传统的JavaScript优化，而是可以经过不同编程语言的优化，从而达到非常高的执行效率。

### 安全性

WASM的安全性在其官网[Security](https://webassembly.org/docs/security/)部分有详细的描述。一个最显著的特点，WASM程序是运行在沙盒中的。一般来说，WASM程序是无法读写除了自身被分配的内存以外的地址的，因此不会干扰外部程序执行。

#### License

<sup>
本仓库遵循<a href="https://creativecommons.org/licenses/by/4.0/">CC-BY-4.0版权协议</a>。
</sup>

<br/>

<sub>
作为<a href="https://copyleft.org/">copyleft</a>的支持者之一，我由衷地欢迎大家积极热情地参与到开源社区中。Happy coding!
</sub>
