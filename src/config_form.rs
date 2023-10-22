use ppaass_agent::config::AGENT_CONFIG;
use yew::prelude::*;

#[derive(Debug)]
pub(crate) struct PpaassConfigurationForm {
    proxy_address: String,
    listening_port: u16,
    user_token: String,
}

#[function_component(ConfigurationForm)]
pub(crate) fn configuration_form() -> Html {
    let configurtion_form = use_state(|| PpaassConfigurationForm {
        proxy_address: match AGENT_CONFIG.get_proxy_addresses() {
            Some(addresses) => addresses
                .iter()
                .map(|addr| addr.to_string())
                .reduce(|l, r| format!("{l},{r}"))
                .unwrap_or("".to_string()),
            None => "".to_string(),
        },
        listening_port: AGENT_CONFIG.get_port(),
        user_token: match AGENT_CONFIG.get_user_token() {
            Some(user_token) => user_token.to_string(),
            None => "".to_string(),
        },
    });

    let configurtion_form_clone = configurtion_form.clone();
    let on_start_btn_click = Callback::from(move |event: MouseEvent| {
        gloo::console::log!(format!(
            "Current configuration form: {configurtion_form_clone:?}, event: {event:?}"
        ));
    });

    html! {
        <section id="configuration_form_pannel">
            <div id="configuration_form_fields">
                <div>
                    <label for="listening port">{"Listinging port"}</label>
                    <input id="listening port" name="listening port" type="text" placeholder={"Linstening port"} value={configurtion_form.listening_port.to_string()} />
                </div>
                <div>
                    <label for="proxy_ip">{"Proxy ip address"}</label>
                    <input id="proxy_ip" name="proxy_ip" type="text" placeholder={"Proxy ip address"} value={configurtion_form.proxy_address.to_string()} />
                </div>
                <div>
                    <label for="user_token">{"User token"}</label>
                    <input id="user_token" name="user_token" type="text" placeholder={"User token"} value={configurtion_form.user_token.to_string()} />
                </div>
            </div>
            <div id="configuration_form_actions">
                <button onclick={on_start_btn_click}>{"Start"}</button>
            </div>
        </section>
    }
}
