use wasm_bindgen::JsCast;
use web_sys::EventTarget;

pub fn expect_target<T: JsCast>(target: Option<EventTarget>) -> Option<T> {
    target.and_then(|t| match t.dyn_into::<T>() {
        Ok(value) => Some(value),
        Err(e) => {
            log::error!("expect_target() failed: {e:?}");

            None
        }
    })
}
