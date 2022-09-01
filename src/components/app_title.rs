use crate::{components::*, utils::init_wasm};
use yew::prelude::*;

#[function_component(AppTitle)]
pub fn app_title() -> Html {
    let is_hidden = !init_wasm::isWindowControlsOverlayVisible();

    if is_hidden {
        return html! {};
    }

    html! {
        <div
            class="fixed px-2"
            style="left: env(titlebar-area-x, 0); top: env(titlebar-area-y, 0); width: env(titlebar-area-width, 100%); height: env(titlebar-area-height, 33px); background-color: #f72405;"
        >
            <article
                class="flex gap-2"
            >
                <img
                    class="p-1"
                    src={ LOGO_URL }
                    style="height: env(titlebar-area-height, 33px); width: env(titlebar-area-height, 33px);"
                />
                <h4
                    class="text-white flex-grow flex justify-center items-center font-mono text-base"
                >
                    { "sneu_rs" }
                </h4>
            </article>
        </div>
    }
}
