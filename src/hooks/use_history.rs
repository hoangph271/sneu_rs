use yew_router::{hooks, prelude::AnyHistory};

pub fn use_history() -> AnyHistory {
    hooks::use_history().unwrap_or_else(|| panic!())
}
