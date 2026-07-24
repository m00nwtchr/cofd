use std::{
	fs::File,
	ops::{Deref, DerefMut},
};

use anyhow::anyhow;
use cofd_schema::book::Book;
use dioxus::prelude::*;
use directories::ProjectDirs;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::router::Route;

pub static DIRS: Lazy<ProjectDirs> =
	Lazy::new(|| ProjectDirs::from("", "", "CofD.tools").expect("Project dirs"));

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Books {
	#[serde(default)]
	pub books: Vec<Book>,
	#[serde(default)]
	pub needs_meta: Vec<String>,
}

impl Books {
	pub fn save(&self) -> anyhow::Result<()> {
		let data_path = DIRS.data_dir().join("books.json");

		serde_json::ser::to_writer_pretty(File::create(data_path)?, self)?;
		Ok(())
	}
}

impl Deref for Books {
	type Target = Vec<Book>;

	fn deref(&self) -> &Self::Target {
		&self.books
	}
}

impl DerefMut for Books {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.books
	}
}

// #[derive(Default, Serialize, Deserialize)]
// pub struct AppState {
// 	#[serde(skip, default)]
// 	pub books: Vec<Book>,
// }

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct AppSettings {}

pub fn use_books() -> Signal<Books> {
	consume_context()
}

pub fn use_app_settings() -> Signal<AppSettings> {
	consume_context()
}

fn read_books() -> anyhow::Result<Books> {
	let data_path: std::path::PathBuf = DIRS.data_dir().join("books.json");
	if !DIRS.data_dir().exists() {
		std::fs::create_dir_all(DIRS.data_dir())?;
		return Err(anyhow!(""));
	}

	if data_path.exists() && data_path.is_file() {
		Ok(serde_json::de::from_reader(File::open(data_path)?)?)
	} else {
		Err(anyhow!(""))
	}
}

fn read_settings() -> anyhow::Result<AppSettings> {
	let config_path = DIRS.config_dir().join("settings.toml");
	if !DIRS.config_dir().exists() {
		std::fs::create_dir_all(DIRS.config_dir())?;
		return Err(anyhow!(""));
	}

	if config_path.exists() {
		Ok(toml::from_str(&std::fs::read_to_string(config_path)?)?)
	} else {
		Err(anyhow!(""))
	}
}

#[component]
pub fn App() -> Element {
	use_context_provider(|| {
		Signal::new(read_books().unwrap_or_else(|err| {
			log::error!("{err}");
			Default::default()
		}))
	});
	use_context_provider(|| Signal::new(read_settings().unwrap_or_default()));

	rsx! {
		document::Stylesheet {
			href: asset!("/assets/tailwind.css")
		},
		Router::<Route> {}
	}
}
