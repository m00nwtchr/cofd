use dioxus::prelude::*;

use crate::{app::use_books, components::NavComponent};

#[component]
pub fn Books() -> Element {
	let mut files_uploaded = use_signal(Vec::new);
	let mut books_context = use_books();

	let upload = move |evt: Event<FormData>| {
		let files = evt.files();
		files_uploaded.with_mut(|files_uploaded| {
			for file in files {
				let file_name = file.name();
				let pending_file_name = file_name.clone();
				let file_path = file.path().to_owned();
				spawn(async move {
					let task = tokio::task::spawn_blocking({
						let file_path = file_path.clone();
						move || cofd_miner::parse_book(&file_path)
					});

					match task.await {
						Ok(Ok(book)) => {
							let mut books = books_context.write();
							books.push(book);
							let _ = books.save();
						}
						Ok(Err(err))
							if matches!(
								err.downcast_ref::<cofd_miner::error::CofDMinerError>(),
								Some(cofd_miner::error::CofDMinerError::NoSuchMeta)
							) =>
						{
							log::warn!(
								"No metadata for {file_path:?}; \
								 run `cargo meta-edit {file_path:?}` to create one."
							);
							books_context.write().needs_meta.push(pending_file_name);
						}
						Ok(Err(err)) => {
							log::error!("Failed to parse {file_path:?}: {err}");
						}
						Err(join_err) => {
							log::error!("Parse task panicked: {join_err}");
						}
					}
				});

				files_uploaded.push(file_name);
			}
		});
	};

	let needs_meta = books_context.read().needs_meta.clone();

	rsx! {
		header {
			NavComponent {}
		}
		main {

			input {
				r#type: "file",
				accept: ".pdf",
				multiple: true,
				onchange: upload,
			},

			div { "progress: {files_uploaded.read().len()}" },

			ul {
				for file in files_uploaded.read().iter() {
					li { "{file}" }
				}
			}

			if !needs_meta.is_empty() {
				div { class: "needs-meta",
					h3 { "Files needing metadata:" }
					p {
						"These PDFs were not parsed because no SourceMeta JSON exists for them. Run "
						code { "cargo meta-edit <pdf-path>" }
						" to create one."
					}
					ul {
						for name in needs_meta.iter() {
							li { "{name}" }
						}
					}
				}
			}
		}
	}
}
