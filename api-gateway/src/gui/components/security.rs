use yew::prelude::*;
use reqwest::Client;

#[function_component(Security)]
pub fn security() -> Html {
    let tls_enabled = use_state(|| false);

    html! {
        <div>
            <h2>{"Security Settings"}</h2>
            <p>{ format!("TLS Enabled: {}", *tls_enabled) }</p>
            <button>{"Enable TLS"}</button>
        </div>
    }
}
