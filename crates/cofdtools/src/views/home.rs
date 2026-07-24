use dioxus::prelude::*;

use crate::components::NavComponent;

#[component]
pub fn Home() -> Element {
	rsx! {
		header {
			NavComponent {}
		}
		main {

		}
	}
}
