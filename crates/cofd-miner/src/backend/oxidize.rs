use std::{collections::BTreeMap, path::Path, result::Result};

use anyhow::anyhow;
use cofd_schema::DOT_CHAR;
use once_cell::sync::Lazy;
use oxidize_pdf::parser::PdfDocument;
use regex::Regex;

use super::PdfText;

static DOT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(&format!("^{DOT_CHAR} ")).unwrap());

const THRESHOLD: f64 = 240.0;

pub fn extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText> {
	let doc = PdfDocument::open(path.as_ref())?;

	let chunks = doc.rag_chunks()?;

	// let reader = PdfReader::open(
	// 	path.as_ref()
	// 		.to_str()
	// 		.ok_or(anyhow!("Path is not valid utf-8 string"))?,
	// )?;

	// let document = reader.into_document();
	// let text = document.extract_text().unwrap();

	let mut pages = BTreeMap::new();

	for chunk in &chunks {
		println!("---\n{}\n---", chunk.text);
		// println!(
		// 	"Chunk {}: pages {:?}, ~{} tokens",
		// 	chunk.chunk_index, chunk.page_numbers, chunk.token_estimate
		// );
		// println!("  Types: {}", chunk.element_types.join(", "));
		// if let Some(heading) = &chunk.heading_context {
		// 	println!("  Section: {}", heading);
		// }

		// Use chunk.full_text for embeddings (includes heading context)
		// Use chunk.text for display (content only)
	}

	// for (i, page_text) in text.iter().enumerate() {
	// 	let mut l_indent = (f64::MAX, f64::MIN);
	// 	let mut r_indent = (f64::MAX, f64::MIN);

	// 	let mut last_y = 0.0;
	// 	let mut blank = false;

	// 	let mut lines = Vec::new();

	// 	for fragment in &page_text.fragments {
	// 		// for line in fragment.lines() {
	// 		let x = fragment.x;
	// 		let y = fragment.y;
	// 		let line = fragment.text.clone();

	// 		let y_shift = (y - last_y).floor();

	// 		if y_shift.abs() > 100.0 {
	// 			blank = y_shift > 0.0; // End of Page Content
	// 		}
	// 		last_y = y;

	// 		if blank || line.trim().chars().all(char::is_numeric) {
	// 			// Some(format!(
	// 			// 	"{min_x}{indent}:BLANK:{}",
	// 			// 	l.chars().filter_map(|c| c.char()).collect::<String>()
	// 			// ));
	// 			continue;
	// 		}

	// 		if x > THRESHOLD {
	// 			if x < r_indent.0 {
	// 				r_indent.0 = x;
	// 			}
	// 		} else if x < l_indent.0 {
	// 			l_indent.0 = x;
	// 		}

	// 		lines.push((x, line));
	// 		// }
	// 	}

	// 	let mut last_x = 0.0;
	// 	let mut last_indent = 0.0;

	// 	let mut last_has_dot = false;
	// 	// let mut dot_line_indent = f32::MAX;
	// 	// let mut dot_paragraph_indent = f32::MAX;
	// 	// let mut pre_dot_indent = f32::MAX;

	// 	let mut last_should_tab = false;
	// 	let mut last_line = String::new();

	// 	let lines = lines
	// 		.into_iter()
	// 		.map(|(x, line)| {
	// 			let min_x = if x < THRESHOLD {
	// 				l_indent.0
	// 			} else {
	// 				r_indent.0
	// 			};

	// 			let dot = DOT_REGEX.is_match(&line);
	// 			let indent = (x - min_x).floor();
	// 			let indent = if dot { indent.max(9.0) } else { indent };

	// 			#[allow(clippy::if_same_then_else, clippy::nonminimal_bool)]
	// 			let should_tab = if indent > last_indent {
	// 				if last_has_dot && !dot { false } else { true }
	// 			} else if indent < last_indent {
	// 				if dot { true } else { false }
	// 			} else if last_line.trim().ends_with(':') && dot {
	// 				true
	// 			} else {
	// 				last_should_tab
	// 			};

	// 			let indent = if indent == 0.0 && should_tab {
	// 				9.0
	// 			} else {
	// 				indent
	// 			};

	// 			last_x = x;

	// 			last_indent = indent;
	// 			last_should_tab = should_tab;
	// 			last_has_dot = dot;
	// 			last_line.clone_from(&line);

	// 			let prefix = if should_tab { "\t" } else { "" };

	// 			#[cfg(debug_assertions)]
	// 			if std::env::var("INDENT_DEBUG").is_ok() {
	// 				format!("{indent}{prefix}{line}")
	// 			} else {
	// 				format!("{prefix}{line}")
	// 			}
	// 			#[cfg(not(debug_assertions))]
	// 			format!("{prefix}{line}")
	// 		})
	// 		.collect();

	// 	pages.insert(i, lines);
	// }

	Ok(pages)
}
