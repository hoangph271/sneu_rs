use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/utils/init_wasm.js")]
extern "C" {
    pub fn isWindowControlsOverlayVisible() -> bool;
}
