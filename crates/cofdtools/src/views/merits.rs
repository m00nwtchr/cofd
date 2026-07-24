use cofd_schema::{
	book::{Book, MeritItem},
	item::{Item, merit::Merit},
};
use dioxus::prelude::*;

use crate::{app::use_books, components::NavComponent};

#[component]
pub fn Merits() -> Element {
	let books = use_books();
	let books = books.read();
	let mut merits: Vec<&MeritItem> = books.iter().flat_map(|book| &book.merits).collect();

	merits.sort_by_key(|merit| &merit.name);
	let mut selected_merit = use_signal(|| 0usize);

	let merit_rendered = if let Some(merit) = merits.get(selected_merit()) {
		let effect = merit.effects.first().cloned().unwrap_or_default();
		rsx! {
			span {
				h1 { "{merit.name}" }
				for paragraph in &merit.description {
					p { class: "indent-4", "{paragraph}" }
				}

				for (idx, paragraph) in merit.effects.iter().enumerate() {
					p { class: "indent-4",
						if idx == 0 {
							span { class:"font-bold", "Effects: "}
						}
						"{paragraph}"
					}
				}
			}
		}
	} else {
		rsx! {
			span {}
		}
	};

	let merits_rendered = merits.iter().enumerate().map(|(i, item)| {
		let dot_rating = item.inner.dot_rating.to_string();
		let prerequisites = item.inner.prerequisites.to_string();

		rsx!(tr {
			onclick: {
				move |_| selected_merit.set(i)
			},
			style: "cursor: pointer;",
			td { "{item.name}" }
			td { "{dot_rating}" }
			td { "{prerequisites}" }
			td { "{item.reference}" }
		})
	});

	rsx! {
		header {
			NavComponent {}
		}
		main {
			class: "container mx-auto",

			div {
				class: "flex flex-row flex-wrap py-4",
				div {
					class: "w-full sm:w-2/3 md:w-3/4 pt-1 px-2",
					table {
						class: "w-full",
						tr {
							th {
								"Name"
							}
							th {
								"Rating"
							}
							th {
								"Prerequisites"
							}
							th {
								"Source"
							}
						}
						{merits_rendered}
					},
				}
				div {
					class: "w-full sm:w-1/3 md:w-1/4 px-2",
					{merit_rendered}
				}
			}
		}
	}
}
