mod constants;
pub mod datetime;
mod dom;
pub mod init_wasm;
pub mod json;
pub mod sneu_api;

pub use constants::*;
pub use datetime::*;
pub use dom::*;
pub use init_wasm::*;

pub const fn no_op() {}

pub fn is_tauri_app() -> bool {
    cfg!(feature = "sneu_tauri")
}
