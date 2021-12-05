use vertigo::{Computed, VDomElement, html, css_fn};

use crate::list;

use super::state::State;

css_fn! { main_div, "
    color: darkblue;
" }

pub fn render(app_state: &Computed<State>) -> VDomElement {
    let state = app_state.get_value();
    let message = state.message.get_value();

    let message_element = if *state.strong.get_value() {
        html! { <strong>{message}</strong> }
    } else {
        html! { <span>{message}</span> }
    };

    let switch = {
        let strong = state.strong.clone();
        move || {
            strong.set_value(
                !*strong.get_value()
            )
        }
    };

    let list_state = &state.list;

    html! {
        <div css={main_div()}>
            "Message to the world: " {message_element} " "
            <button on_click={switch}>"Switch"</button>
            {list::render(&list_state)}
        </div>
    }
}
