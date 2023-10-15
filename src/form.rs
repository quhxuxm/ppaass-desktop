use yew::prelude::*;

#[function_component(ConfigurationForm)]
pub(crate) fn configuration_form() -> Html {
    let onclick = |event: MouseEvent| {
        gloo::console::log!(format!("Receive mouse event: {event:?}"));
    };
    html! {
        <section id="configuration_form_pannel">
            <div id="configuration_form_fields">
                <div>
                    <label for="listening port">{"Listinging port"}</label>
                    <input id="listening port" name="listening port" type="text" placeholder={"Linstening port"} />
                </div>
                <div>
                    <label for="proxy_ip">{"Proxy ip address"}</label>
                    <input id="proxy_ip" name="proxy_ip" type="text" placeholder={"Proxy ip address"} />
                </div>
                <div>
                    <label for="user_token">{"User token"}</label>
                    <input id="user_token" name="user_token" type="text" placeholder={"User token"}/>
                </div>
            </div>
            <div id="configuration_form_actions">
                <button {onclick}>{"Start"}</button>
            </div>
        </section>
    }
}
