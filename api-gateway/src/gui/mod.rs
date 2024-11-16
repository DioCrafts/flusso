pub mod components;
pub mod gui_server;

pub use gui_server::start_gui_server;

use yew::prelude::*;
use components::{Dashboard, Gateways, Routes, Metrics, Security};


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <Dashboard />
            <Gateways />
            <Routes />
            <Metrics />
            <Security />
        </div>
    }
}

