use wasm_bindgen::prelude::*;
use web_sys::File;
use web_sys::HtmlAudioElement;

#[wasm_bindgen(module = "/src/utils/init_wasm.js")]
extern "C" {
    pub fn isWindowControlsOverlayVisible() -> bool;

    pub fn setSrcObject(audio_el: &HtmlAudioElement, blob: &File);
}
