use yew::prelude::*;

#[function_component(Metrics)]
pub fn metrics() -> Html {
    html! {
        <div>
            <h2>{"Metrics"}</h2>
            <p>{"Traffic Overview: 1000 requests served."}</p>
            <p>{"Error Rate: 0.02%"}</p>
        </div>
    }
}
