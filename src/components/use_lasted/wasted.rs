use yew::prelude::*;

#[function_component(Wasted)]
pub fn wasted() -> Html {
    html! {
        <a
            className="Wasted"
            href="https://github.com/hoangph271/useLasted"
            style="color: white"
        >
            { "@source_code" }
        </a>
    }
}
