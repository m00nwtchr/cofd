#[cfg(feature = "mupdf")]
mod mupdf;

#[cfg(feature = "oxidize")]
mod oxidize;

use std::collections::BTreeMap;

#[cfg(feature = "mupdf")]
pub use mupdf::extract_pages;
#[cfg(feature = "oxidize")]
pub use oxidize::extract_pages;

pub type PdfText = BTreeMap<usize, Vec<String>>;
