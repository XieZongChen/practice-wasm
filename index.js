/**
 * 引入编译后的文件
 * 通常 wasm-bindgen 会在 Cargo.toml 所在目录生成一个 pkg 文件夹，里面包含了编译后的文件
 * 所要引入的文件名随 Cargo.toml 中的 name 配置项而定
 */
const rust = import('./pkg/wasm_rust.js');

rust
  .then(m => m.hello_world('World!'))
  .catch(console.error);