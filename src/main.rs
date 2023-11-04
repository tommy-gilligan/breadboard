#![feature(int_roundings)]
mod app;
mod breadboard;
mod view;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
