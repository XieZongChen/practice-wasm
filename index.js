// 直接引入了，刚才编译后的文件
const rust = import('./pkg/wasm_rust.js');

rust
  .then(m => m.hello_world('World!'))
  .catch(console.error);