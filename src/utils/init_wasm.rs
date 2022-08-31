use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/utils/init_wasm.js")]
extern "C" {
    fn initWasm() -> String;
}

pub fn init_wasm() {
    initWasm();
}
