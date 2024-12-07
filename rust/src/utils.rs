use js_sys::Array;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


pub fn log_vec(vec: &Vec<Vec<u8>>) {
    let js_array = Array::new();
    for row in vec {
        let js_row = Array::new();
        for cell in row {
            js_row.push(&JsValue::from(*cell));
        }
        js_array.push(&js_row);
    }

    console::log_1(&js_array);
}