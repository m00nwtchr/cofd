use dioxus::prelude::*;

use crate::views::*;

#[derive(Routable, Clone)]
pub enum Route {
	#[route("/")]
	Home,
	#[route("/books")]
	Books,
	#[route("/merits")]
	Merits,

	#[route("/:..route")]
	NotFound { route: Vec<String> },
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
	rsx! {
		h1 { "Page not found" }
	}
}
