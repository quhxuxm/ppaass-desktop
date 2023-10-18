mod app;
mod config_form;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
