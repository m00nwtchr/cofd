#[cfg(all(feature = "mupdf", not(feature = "oxidize")))]
mod mupdf;

#[cfg(feature = "oxidize")]
mod oxidize;

use std::collections::BTreeMap;

#[cfg(all(feature = "mupdf", not(feature = "oxidize")))]
pub use mupdf::extract_pages;
#[cfg(feature = "oxidize")]
pub use oxidize::extract_pages;

pub type PdfText = BTreeMap<usize, Vec<String>>;
