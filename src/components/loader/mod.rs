use yew::prelude::*;

#[function_component(Loader)]
pub fn loader() -> Html {
    html! {
        <div
            class="w-24 border-8 border-zinc-300 border-t-teal-300 aspect-square rounded-full animate-spin"
        />
    }
}

pub fn with_loader<T>(option: Option<T>, render: impl Fn(T) -> Html) -> Html {
    match option {
        Some(data) => html! { render(data) },
        None => html! { <Loader /> },
    }
}
