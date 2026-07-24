#![allow(non_snake_case)]

mod app;
mod components;
mod router;
mod views;

#[cfg(not(target_arch = "wasm32"))]
static CSS: &str = include_str!("../public/tailwind.css");

fn main() {
	env_logger::init();

	dioxus::launch(app::App);
}
