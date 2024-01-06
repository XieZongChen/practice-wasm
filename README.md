# 练习 wasm 项目

## 命令

**在 wasm-rust 中**

```bash
# --target 标志用于指定要构建的目标平台
# wasm32-unknown-unknown 是指定了 WebAssembly 目标平台，这告诉 Cargo 生成「适用于 WebAssembly 的二进制文件」，而不是生成本地平台的二进制文件
cargo build --target wasm32-unknown-unknown
```
> 当运行这个命令后，Cargo 会使用 Rust 编译器（Rustc）以及与 WebAssembly 相关的工具链，将 Rust 代码编译为 WebAssembly 格式的二进制文件。这个生成的 Wasm 文件可以在浏览器中运行，或与其他支持 WebAssembly 的环境一起使用。