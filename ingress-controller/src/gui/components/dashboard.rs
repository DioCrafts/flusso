use yew::prelude::*;
use reqwest::Client;
use serde_json::json;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let ingresses_count = use_state(|| 0);  // Total de Ingresses
    let routes_count = use_state(|| 0);     // Total de Routes
    let server_status = use_state(|| String::from("loading..."));  // Estado del servidor

    {
        let ingresses_count = ingresses_count.clone();
        let routes_count = routes_count.clone();
        let server_status = server_status.clone();

        // Cambiar use_effect_with a use_effect (sin dependencias)
        use_effect(move || {
            let client = Client::new();

            wasm_bindgen_futures::spawn_local(async move {
                // Obtener el número de Ingresses
                let ingresses_response: Vec<String> = client
                    .get("http://localhost:8080/api/ingresses")  // URL de la API de Ingresses
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                ingresses_count.set(ingresses_response.len());

                // Obtener el número de Routes
                let routes_response: Vec<String> = client
                    .get("http://localhost:8080/api/routes")  // URL de la API de Routes
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                routes_count.set(routes_response.len());

                // Simulando una verificación del estado del servidor
                server_status.set(String::from("running"));
            });

            || ()  // Aquí no es necesario pasar dependencias adicionales
        });
    }

    html! {
        <div>
            <h2>{"Dashboard"}</h2>
            <p>{"Server Status: "}{(*server_status).clone()}</p>
            <p>{"Total Ingresses: "}{*ingresses_count}</p>
            <p>{"Total Routes: "}{*routes_count}</p>
        </div>
    }
}
