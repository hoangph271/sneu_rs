mod app;
mod components;
mod providers;
mod router;
mod utils;
mod views;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::AppWithContext>();
}
