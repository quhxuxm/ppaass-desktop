mod app;
mod component;
mod value;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
