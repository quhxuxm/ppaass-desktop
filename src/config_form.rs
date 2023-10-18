use yew::prelude::*;

pub(crate) struct PpaassConfiguration {
    proxy_address: String,
    listening_port: u16,
    user_token: String,
}

#[function_component(ConfigurationForm)]
pub(crate) fn configuration_form() -> Html {
    let configurtion_form = use_state(|| PpaassConfiguration {
        proxy_address: "64.176.193.76".to_string(),
        listening_port: 80,
        user_token: "user1".to_string(),
    });

    let onclick = |event: MouseEvent| {
        gloo::console::log!(format!("Receive mouse event: {event:?}"));
    };

    html! {
        <section id="configuration_form_pannel">
            <div id="configuration_form_fields">
                <div>
                    <label for="listening port">{"Listinging port"}</label>
                    <input id="listening port" name="listening port" type="text" placeholder={"Linstening port"} value={&configurtion_form.listening_port}/>
                </div>
                <div>
                    <label for="proxy_ip">{"Proxy ip address"}</label>
                    <input id="proxy_ip" name="proxy_ip" type="text" placeholder={"Proxy ip address"} value={&configurtion_form.proxy_address} />
                </div>
                <div>
                    <label for="user_token">{"User token"}</label>
                    <input id="user_token" name="user_token" type="text" placeholder={"User token"} value={&configurtion_form.user_token}/>
                </div>
            </div>
            <div id="configuration_form_actions">
                <button {onclick}>{"Start"}</button>
            </div>
        </section>
    }
}
