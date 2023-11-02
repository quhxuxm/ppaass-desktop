use stylist::{yew::styled_component, Style};
use yew::{html, Html, Properties};

const STYLE_FILE_CONTENT: &str = include_str!("input.css");

#[derive(Properties, PartialEq)]
pub struct InputProp {
    pub label: String,
    pub value: String,
    pub place_holder: String,
    pub id: String,
}

#[styled_component(Input)]
pub fn input(prop: &InputProp) -> Html {
    let style = Style::new(STYLE_FILE_CONTENT).unwrap();
    html! {
        <section class={style}>
            <label for={prop.id.clone()}>{prop.label.clone()}</label>
            <input id={prop.id.clone()}
                type="text"
                placeholder={prop.place_holder.clone()}/>
        </section>
    }
}
