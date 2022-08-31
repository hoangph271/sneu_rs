mod app;
mod components;
mod hooks;
mod providers;
mod router;
mod theme;
mod utils;
mod views;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    utils::init_wasm();

    yew::start_app::<app::AppWithContext>();

    log::info!("`start_app()`ed")
}
