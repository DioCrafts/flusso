use yew::prelude::*;
use reqwest::Client;

#[function_component(Ingresses)]
pub fn ingresses() -> Html {
    let ingresses = use_state(|| Vec::new());

    {
        let ingresses = ingresses.clone();
        use_effect_with_deps(move |_| {
            let client = Client::new();
            wasm_bindgen_futures::spawn_local(async move {
                let response: Vec<String> = client
                    .get("http://localhost:8080/api/ingresses")  // URL de la API backend que devuelve los Ingresses
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                ingresses.set(response);
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <h2>{"Ingresses Management"}</h2>
            <ul>
                { for ingresses.iter().map(|ingress| html! { <li>{ ingress }</li> }) }
            </ul>
            <button>{"Create New Ingress"}</button>
        </div>
    }
}
