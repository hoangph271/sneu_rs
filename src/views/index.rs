use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq, Default)]
pub struct IndexProps {}

#[function_component(Index)]
pub fn index(_: &IndexProps) -> Html {
    html! {
        <form
            action="http://localhost:8000/api/v1/users/signin"
            method="post"
            style="display: flex; flex-direction: column; gap: 1rem; align-items: center; justify-content: center;"
        >
            <label>
                <span>{ "Username:" }</span>
                <input type="text" />
            </label>
            <label>
                <span>{ "Password:" }</span>
                <input type="password" />
            </label>
            <button type="submit">{ "Sign in" }</button>
        </form>
    }
}
