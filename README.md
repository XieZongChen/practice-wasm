# 练习 wasm 项目

### 项目

```bash
├── src              # rust 所在目录
├── index.js         # web 脚本入口
├── Cargo.toml       # rust 项目配置文件
└── package.json     # web 项目配置文件
```

### 指令

#### 对于 rust

```bash
# --target 标志用于指定要构建的目标平台
# wasm32-unknown-unknown 是指定了 WebAssembly 目标平台，
# 这告诉 Cargo 生成「适用于 WebAssembly 的二进制文件」，而不是生成本地平台的二进制文件
cargo build --target wasm32-unknown-unknown
```

> 当运行这个命令后，Cargo 会使用 Rust 编译器（Rustc）以及与 WebAssembly 相关的工具链，将 Rust 代码编译为 WebAssembly 格式的二进制文件。这个生成的 Wasm 文件可以在浏览器中运行，或与其他支持 WebAssembly 的环境一起使用。

#### 对于 web

```bash
# 启动 dev 环境 localhost:8080
npm run serve

# 构建程序
npm run build
```

### 原理探析

在使用 cargo 和 wasm_bindgen 编译源代码时，会在 pkg 文件中「自动生成」以下文件：

- wasm_rust_bg.js
- wasm_rust_bg.wasm
- wasm_rust_bg.wasm.d.ts
- wasm_rust.js
- wasm_rust.d.ts
- package.json

> wasm_rust_bg.js 文件是由 wasm-bindgen 自动生成的，它包含了用于将 DOM 和 JavaScript 函数导入到 Rust 中的 JavaScript 粘合代码。它还在生成的 WebAssembly 函数上向 JavaScript 公开了 API。

Rust WebAssembly 专注于将 WebAssembly 与现有的 JavaScript 应用程序集成在一起。为了实现这一目标，我们需要在 JavaScript 和 WebAssembly 函数之间「传递不同的值、对象或结构。这并不容易，因为需要协调两个不同系统的不同对象类型」。

当前 WebAssembly 仅支持「整数」和「浮点数」，不支持字符串。这意味着我们不能简单地将字符串传递给 WebAssembly 函数。要将字符串传递给 WebAssembly，我们需要「将字符串转换为数字」（请注意在 webpack.config.js 中指定的 TextEncoder API），将这些数字放入 WebAssembly 的内存空间中，最后「返回一个指向字符串的指针」给 WebAssembly 函数，以便在 JavaScript 中使用它。在最后，我们需要释放 WebAssembly 使用的字符串内存空间。

在 wasm_rust_bg.js 中可以找到一个 hello_world 函数，这正是自动执行的操作。hello_world 函数首先调用 passStringToWasm。这个函数在 WebAssembly 中「创建一些内存空间」，将我们的字符串转换为数字，将数字写入内存空间，并返回一个指向字符串的指针。然后将指针传递给 wasm.hello_world 来执行 JavaScript 的 alert。最后，wasm.__wbindgen_free 释放了内存。