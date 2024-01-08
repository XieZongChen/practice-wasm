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
 *
 * Web 具有大量 API，从 DOM 操作到 WebGL 再到 Web Audio 等等。
 * 因此，如果我们的 Rust WebAssembly 程序增长，并且我们需要对 Web API 进行多次不同的调用，我们将需要花时间编写大量的 extern 代码。
 * 我们可以使用 web-sys 充当 wasm-bindgen 的前端，为所有 Web API 提供原始绑定，从而不必编写 extern 代码。
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

#[wasm_bindgen]
pub fn make_the_window_small() {
    // 调整窗口大小为 500px * 500px
    let window = web_sys::window().unwrap();
    window.resize_to(500, 500).expect("无法调整窗口大小");
}
