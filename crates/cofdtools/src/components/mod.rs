use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn NavComponent() -> Element {
	rsx! {
		nav {
			Link { to: Route::Home {}, "Home" }
			Link { to: Route::Books {}, "Books" }
			Link { to: Route::Merits {}, "Merits" }
		}
	}
}
