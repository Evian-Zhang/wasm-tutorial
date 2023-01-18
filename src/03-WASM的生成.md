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

首先是`#[no_mangle]`。这个属性和`#[export_name = "xxxx"]`一样，在Rust官方文档的[Application Binary Interface](https://doc.rust-lang.org/reference/abi.html)中有说明。这些属性一般而言，是在我们用Rust写一些提供给别的语言调用的库函数时使用，用来关闭命名修饰（Name mangling）。但是在目标平台为WASM时，语义会有少许变化。在这里，使用`#[no_mangle]`属性，除了告诉编译器，生成的函数名字在二进制层面就叫`add`以外，还有一个作用，是让编译器**导出**这个符号。我们知道，当我们使用一个Rust crate的时候，只能使用其中用`pub`修饰的函数。但是，`pub`只是Rust语义层面的。在WASM层面，我们必须使用`#[no_mangle]`或者`#[export_name = "xxxx"]`这个属性，才能确保编译器确实导出了这个函数。

其次是`extern "C"`。同样地，这个修饰符在一般情况下，是用来告诉编译器，这个函数的ABI采用C语言的ABI，以便在生成二进制库的时候可以被别的编程语言调用。而在生成目标为WASM的情况下，目前Rust的`extern "wasm"`[还没有稳定](https://github.com/rust-lang/rust/issues/83788)，所以暂时也是使用`extern "C"`来声明ABI。

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
