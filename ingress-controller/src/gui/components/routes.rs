use yew::prelude::*;
use reqwest::Client;
use serde_json::json;

#[function_component(Routes)]
pub fn routes() -> Html {
    let routes = use_state(|| Vec::new()); // Usamos use_state para mantener las rutas

    {
        let routes = routes.clone();
        use_effect(move || {
            let client = Client::new();

            wasm_bindgen_futures::spawn_local(async move {
                let response: Vec<String> = client
                    .get("http://localhost:8080/api/routes")  // URL de la API de Routes
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                routes.set(response);
            });

            || ()  // No es necesario pasar dependencias adicionales
        });
    }

    html! {
        <div>
            <h2>{"Routes Management"}</h2>
            <ul>
                { for routes.iter().map(|route| html! { <li>{ route }</li> }) }
            </ul>
            <button>{"Create New Route"}</button>
        </div>
    }
}
