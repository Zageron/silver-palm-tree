use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    using_web_sys();
}

fn using_web_sys() {
    use web_sys::console;

    unsafe {
        console::log_1(&"Hello World!".into());
    }
}
