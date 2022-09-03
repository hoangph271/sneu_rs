use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

pub fn expect_target<T: JsCast>(target: Option<EventTarget>) -> Option<T> {
    target.and_then(|t| match t.dyn_into::<T>() {
        Ok(value) => Some(value),
        Err(e) => {
            log::error!("expect_target() failed: {e:?}");

            None
        }
    })
}

pub fn expect_input_target(target: Option<EventTarget>) -> Option<HtmlInputElement> {
    expect_target(target)
}
