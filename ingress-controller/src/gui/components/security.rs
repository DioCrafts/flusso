use yew::prelude::*;
use reqwest::Client;
use serde_json::json;

#[function_component(Security)]
pub fn security() -> Html {
    let tls_enabled = use_state(|| false);  // Estado para saber si TLS está habilitado

    let toggle_tls = {
        let tls_enabled = tls_enabled.clone();
        Callback::from(move |_| {
            let new_status = !*tls_enabled;
            tls_enabled.set(new_status);

            // Enviar una solicitud para actualizar la configuración de TLS
            let client = Client::new();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = client
                    .post("http://localhost:8080/api/security/tls")  // URL de la API de TLS
                    .json(&json!({ "enabled": new_status }))
                    .send()
                    .await;
            });
        })
    };

    html! {
        <div>
            <h2>{"Security Settings"}</h2>
            <p>{ format!("TLS Enabled: {}", *tls_enabled) }</p>
            <button onclick={toggle_tls}>{ if *tls_enabled { "Disable TLS" } else { "Enable TLS" } }</button>
        </div>
    }
}
