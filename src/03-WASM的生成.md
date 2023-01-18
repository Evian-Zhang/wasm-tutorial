# WASM的生成

在我们学习底层语言，如LLVM IR、AMD64和AArch64汇编等的过程中，一个非常有效的方法是，将自己熟悉的高级语言编译为相应的底层语言，从而了解不同的语义是怎样在底层实现的。因此，我们在学习WASM的过程中，首先也可以先学习一下如何由高级语言生成WASM。

目前最主流的生成WASM的高级语言是Rust和C/C++，因此，本章也主要以这两个语言为例说明。

## Rust程序生成WASM

我们可以用`rustup`程序来管理Rust语言能够生成的目标平台，例如，使用

```shell
rustup target list
```

可以查看目前Rust支持生成哪些平台的程序。我们可以在这其中看到`wasm32-unknown-unknown`和`wasm32-wasi`。目前，我们需要`wasm32-unknown-unknown`。因此，我们使用

```shell
rustup target add wasm32-unknown-unknown
```

下载安装相应的组件，随后，我们就可以使用Rust生成WASM程序了。

### 基础方法

我们使用

```shell
cargo new --lib rust-wasm-adder
```

生成一个默认的Rust库`rust-wasm-adder`。

首先，我们需要修改`Cargo.toml`，在其中加上

```toml
[lib]
crate-type = ["cdylib"]
```

随后，在`lib.rs`中写上

```rust, ignore
#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}
```

然后，使用

```shell
cargo build --release --target wasm32-unknown-unknown
```

编译后，就可以在`target/wasm32-unknown-unknown/release/`目录下找到`rust_wasm_adder.wasm`文件，这就是我们生成的WASM程序。这个WASM程序提供了`add`函数，我们可以使用Hello world一章中提供的方法，在Web或者后端将这个WASM作为库使用，来验证我们确实生成成功了。

关于这段Rust代码，有几点值得指出说明。

#### `#[no_mangle]`

首先是`#[no_mangle]`。这个属性和`#[export_name = "xxxx"]`一样，在Rust官方文档的[Application Binary Interface](https://doc.rust-lang.org/reference/abi.html)中有说明。这些属性一般而言，是在我们用Rust写一些提供给别的语言调用的库函数时使用，用来关闭命名修饰（Name mangling）。但是在目标平台为WASM时，语义会有少许变化。在这里，使用`#[no_mangle]`属性，除了告诉编译器，生成的函数名字在二进制层面就叫`add`以外，还有一个作用，是让编译器**导出**这个符号。我们知道，当我们使用一个Rust crate的时候，只能使用其中用`pub`修饰的函数。但是，`pub`只是Rust语义层面的。在WASM层面，我们必须使用`#[no_mangle]`或者`#[export_name = "xxxx"]`这个属性，才能确保编译器确实导出了这个函数。

#### `extern "C"`

其次是`extern "C"`。同样地，这个修饰符在一般情况下，是用来告诉编译器，这个函数的ABI采用C语言的ABI，以便在生成二进制库的时候可以被别的编程语言调用。而在生成目标为WASM的情况下，目前Rust的`extern "wasm"`[还没有稳定](https://github.com/rust-lang/rust/issues/83788)，所以暂时也是使用`extern "C"`来声明ABI。

#### `usize`

还有一点值得注意，我们之前提到，目前通用的WASM程序可以看做32位平台，因为其内存最多只有32位。所以，`usize`在这个平台下实际上是`u32`。我们通过

```shell
wasm2wat target/wasm32-unknown-unknown/release/rust_wasm_adder.wasm -o rust_wasm_adder.wat
```

也可以看到这一段代码：

```wasm
(func $add (type 0) (param i32 i32) (result i32)
  local.get 1
  local.get 0
  i32.add)
```

从这也可以看出来，确实参数确实是32位（关于WASM中的`i32`和Rust中的`u32`的关系，之后我们会解释）。

#### 导出函数

还有一点值得指出的是，上面这些额外的修饰，只有在该函数需要被导出时才需要使用。也就是说，如果我们写的Rust程序内部有一些内部的函数，不需要在WASM中导出，用来被别的语言使用，那么就不需要加这些额外的修饰。

#### 系统交互

此外，我们之前提到过，WASM本身提供的基础指令，是没有办法进行系统调用的。WASI提供了一套与系统交互的接口，供WASM来使用。因此，当我们Rust程序的目标平台是`wasm32-unknown-unknown`时（也就是说，不使用WASI），是没有办法，在不导入别的函数的情况下，直接进行系统操作的。因此，如果我们的`add`函数写成

```rust, ignore
#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    println!("This is add in WASM");
    left + right
}
```

在执行这个WASM模块的时候，是不会产生任何输出的。这是因为`println`宏需要对`stdout`进行写操作，而这是WASI提供的能力，因此是做不到输出的。

### `wasm-bindgen`

从上面的讨论我们可以知道，Rust本身虽然支持直接生成WASM程序，但是目前还有一些功能不稳定，例如`extern "wasm"`不稳定所以只能使用`extern "C"`。因此，在使用Rust编写WASM程序的过程中，往往更广泛使用的是一个叫[`wasm-bindgen`](https://crates.io/crates/wasm-bindgen)的库。这个库有其[官方文档](https://rustwasm.github.io/docs/wasm-bindgen/)。

最基础地，`wasm-bindgen`提供了一个宏，让我们直接定义需要导出的WASM函数：

```rust, ignore
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

通过`#[wasm_bindgen]`这个宏，我们不需要再手动写之前很长的一大堆`#[no_mangle]`之类的东西。同时，这样的封装也有助于向后兼容。之后`extern "wasm"`稳定之后，只需要`wasm-bindgen`这个库内部改动一下，就可以适配新版本，而不需要开发者再手动修改了。

此外，这个库还有一个很重要的目的，是解决了WASM与JavaScript交互的问题。以其README里的代码简单举个例子：

```rust, ignore
use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

之前提到，不借助WASI，WASM的程序是没办法再不导入函数的情况下与系统交互的。而这里，我们使用`#[wasm_bindgen]`这个宏，就可以在WASM之中使用`window.alert`函数了！

关于这个功能，之后在介绍WASM的函数导入与导出时，会再详细介绍。

## C程序生成WASM

毫无疑问，C/C++是目前使用平台最广的编程语言。因此，我们也可以将C/C++程序生成WASM。这里以C程序为例。

### Clang

第一种方案，与Rust类似，使用LLVM后端来做代码生成。因此，对应的编译器为Clang。

我们编写一个简单的程序`add.c`：

```c
int add(int left, int right) __attribute__((export_name("add"))) {
    return left + right;
}
```

使用如下命令生成WASM程序：

```shell
clang \
  --target=wasm32-unknown-unknown -nostdlib \
  -O3 \
  -Wl,--no-entry -Wl,--export-dynamic \
  -o adder.wasm \
  adder.c
```

生成的`adder.wasm`也可以像Hello world一章中放在Web中或者后端程序中来检验。

首先，我们使用了`--target=wasm32-unknown-unknown`来声明生成的目标平台，同时由于WASM程序不会使用系统的libc，也不会使用系统的crt等等，所以我们也需要使用`-nostdlib`来关闭对这些系统库的链接。

此外，我们使用了`-Wl`选项。这个选项的意思是将后续的参数传递给链接器。这里我们传递给链接器的参数有`--no-entry`和`--export-all`。

我们之前指出过，不使用WASI的WASM程序往往都是库程序，而不会直接作为独立的可执行程序。因此，也就不会有`_start`、`main`等entrypoint。所以我们需要传递给链接器`--no-entry`参数，让它别找entrypoint了。

与Rust类似，我们也可以控制在WASM中导出的函数。当我们给链接器传递[`export-dynamic`](https://lld.llvm.org/WebAssembly.html#cmdoption-export-dynamic)参数时，在程序中以[`export_name`](https://clang.llvm.org/docs/AttributeReference.html#export-name)属性修饰的函数会被导出，而其余的函数是不会被导出的。在我们的例子中，使用了`__attribute__((export_name("add")))`，因此这个函数会以"add"的名字导出。

### Emscripten

之前了解WASM的开发者，想必一定听说过Emscripten的大名。Emscripten有悠久的历史，WASM的发展与其有很多很多的关系。简单来说，Emscripten就是提供了一整套工具链，让我们可以将C/C++程序编译为WASM程序。

在早期，Emscripten的工作流程是，使用emcc将C/C++代码编译为LLVM IR，使用fastcomp将LLVM IR编译为asm.js语言（一种类似JavaScript的底层语言），然后使用我们之前提到的Binaryen工具链中的`asm2wasm`工具，将asm.js代码编译为WASM代码。这一过程详见[I don't know how Binaryen is used by Emscripten](https://9to5tutorial.com/i-don-t-know-how-binaryen-is-used-by-emscripten)。

而现在，Emscripten的工作流程是，使用emcc编译为LLVM IR之后，直接使用LLVM的wasm后端来生成wasm文件。Emscripten也和`wasm-bindgen`类似，提供了一些C/C++与系统交互的接口。

Emscripten使用起来更为简单，我们编写`adder.c`:

```c
#include <emscripten.h>

EMSCRIPTEN_KEEPALIVE int add(int left, int right) {
    return left + right;
}
```

随后使用

```shell
emcc adder.c -O3 -o adder.wasm
```

即可生成一个WASM程序。

这里，我们就不再需要Clang的`export_name`属性，而是直接使用`EMSCRIPTEN_KEEPALIVE`宏即可。
