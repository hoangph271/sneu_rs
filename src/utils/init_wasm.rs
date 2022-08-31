use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/utils/init_wasm.js")]
extern "C" {
    fn name() -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn init_wasm() {
    log(&format!("{}", name()));
}
