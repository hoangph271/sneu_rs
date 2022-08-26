mod views;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<views::Index>();
}
