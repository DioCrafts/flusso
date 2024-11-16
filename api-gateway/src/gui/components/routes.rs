use yew::prelude::*;
use reqwest::Client;

#[function_component(Routes)]
pub fn routes() -> Html {
    let routes = use_state(|| Vec::new());

    {
        let routes = routes.clone();
        use_effect_with((), move |_| {
            let client = Client::new();
            wasm_bindgen_futures::spawn_local(async move {
                let response: Vec<String> = client
                    .get("http://localhost:8080/api/routes")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                routes.set(response);
            });
            || ()
        });
    }

    html! {
        <div>
            <h2>{"Routes"}</h2>
            <ul>
                { for routes.iter().map(|route| html! { <li>{ route }</li> }) }
            </ul>
            <button>{"Create New Route"}</button>
        </div>
    }
}
