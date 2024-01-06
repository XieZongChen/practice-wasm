/**
 * 这一行声明了对 wasm_bindgen 库的依赖。
 * wasm_bindgen 是一个 Rust 库，用于构建 Wasm 模块并提供与 JavaScript 的互操作性。
 * 在 Rust 当中，库被称为 crates，因为我们使用的是一个外部库，所以有 extern。
 */
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

/**
 * 导入 'window.alert'
 * 在 #[] 中的内容叫做 "属性"，并以某种方式改变下面的语句。
 * wasm_bindgen 是一个「属性标记」，用于指定与 WebAssembly 互操作相关的特性。
 * 这里声明了一个「外部函数」alert，它使用 extern "C" 指定了 C ABI（应用二进制接口），这意味着它「可以与C语言进行交互」。
 */
#[wasm_bindgen]
extern "C" {
    // 这个 alert 函数没有在 Rust 中实现，而是在 JavaScript 中实现，用于在浏览器中显示警告框
    fn alert(s: &str);
}

/**
 * 导出一个 'hello_world' 函数
 * 被标记为 wasm_bindgen，这意味着它「可以被JavaScript调用」
 */
#[wasm_bindgen]
pub fn hello_world(name: &str) {
    // 这里调用的是上面导入的 alert 函数
    alert(&format!("Hello World : {}!", name))
}