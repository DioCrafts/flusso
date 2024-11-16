use yew::prelude::*;
use reqwest::Client;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let gateways = use_state(|| Vec::new()); // Usamos use_state para mantener el estado

    {
        let gateways = gateways.clone();
        use_effect_with((), move |_| {
            let client = Client::new();
            wasm_bindgen_futures::spawn_local(async move {
                let response: Vec<String> = client
                    .get("http://localhost:8080/api/gateways") // Ajusta la URL de tu backend
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                gateways.set(response);
            });
            || ()
        });
    }

    html! {
        <div>
            <h2>{"Dashboard"}</h2>
            <ul>
                { for gateways.iter().map(|gateway| html! { <li>{ gateway }</li> }) }
            </ul>
        </div>
    }
}
