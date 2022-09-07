use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct LoaderProps {
    #[prop_or_default]
    pub size: String,
}

#[function_component(Loader)]
pub fn loader(props: &LoaderProps) -> Html {
    let LoaderProps { size } = props;

    let size = if size.is_empty() { "w-24" } else { size }.to_owned();

    html! {
        <div
            class={ format!("{size} border-8 border-zinc-300 border-t-teal-300 aspect-square rounded-full animate-spin") }
        />
    }
}

pub fn with_loader<T>(option: Option<T>, render: impl Fn(T) -> Html) -> Html {
    match option {
        Some(data) => html! { render(data) },
        None => html! { <Loader /> },
    }
}
