use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::component::input::Input;
use crate::value::PpaassConfigurationForm;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct StartServerArg {
    configuration: PpaassConfigurationForm,
}

#[function_component(App)]
pub fn app() -> Html {
    let agent_port_field_ref = use_node_ref();
    let proxy_address_field_ref = use_node_ref();
    let user_token_field_ref = use_node_ref();

    let onclick_callback = {
        let agent_port_field_ref = agent_port_field_ref.clone();
        let proxy_address_field_ref = proxy_address_field_ref.clone();
        let user_token_field_ref = user_token_field_ref.clone();
        Callback::from(move |_event: MouseEvent| {
            let agent_port =
                if let Some(agnet_port_field) = agent_port_field_ref.cast::<HtmlInputElement>() {
                    match agnet_port_field.value().parse::<u16>() {
                        Ok(agnet_port) => agnet_port,
                        Err(_) => {
                            return;
                        }
                    }
                } else {
                    return;
                };
            let proxy_address = if let Some(proxy_address_field) =
                proxy_address_field_ref.cast::<HtmlInputElement>()
            {
                proxy_address_field.value()
            } else {
                return;
            };

            let user_token =
                if let Some(user_token_field) = user_token_field_ref.cast::<HtmlInputElement>() {
                    user_token_field.value()
                } else {
                    return;
                };
            let start_server_arg = StartServerArg {
                configuration: PpaassConfigurationForm {
                    agent_port,
                    proxy_address,
                    user_token,
                },
            };

            spawn_local(async move {
                let start_server_result = invoke(
                    "start_server",
                    match serde_wasm_bindgen::to_value(&start_server_arg) {
                        Ok(start_server_arg) => start_server_arg,
                        Err(_) => return,
                    },
                )
                .await;
                gloo::console::log!(format!("{start_server_result:?}"));
            });
        })
    };

    html! {
        <main>
            <Input label={"Agent port number:"} value={"123u16"} place_holder={"Agent port"} id="agent_port"/>
        </main>
    }
}
