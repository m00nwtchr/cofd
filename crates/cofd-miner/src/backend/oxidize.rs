use std::{collections::BTreeMap, path::Path, sync::LazyLock};

use anyhow::Context;
use cofd_schema::DOT_CHAR;
use oxidize_pdf::parser::PdfDocument;
use oxidize_pdf::text::ExtractionOptions;
use regex::Regex;

use super::PdfText;

static DOT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(&format!("^{DOT_CHAR} ")).unwrap());

const THRESHOLD: f64 = 240.0;

pub fn extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText> {
	let path = path.as_ref();
	let doc = PdfDocument::open(path).with_context(|| format!("opening {}", path.display()))?;

	let options = ExtractionOptions {
		sort_by_position: true,
		..Default::default()
	};

	let page_count = doc
		.page_count()
		.with_context(|| format!("reading page count for {}", path.display()))?;

	let mut pages: PdfText = BTreeMap::new();

	for i in 0..page_count {
		let extracted = doc
			.extract_text_from_page_with_options(i, options.clone())
			.with_context(|| format!("extracting page {i} from {}", path.display()))?;

		let mut l_indent = (f64::MAX, f64::MIN);
		let mut r_indent = (f64::MAX, f64::MIN);

		let mut last_y = 0.0;
		let mut blank = false;

		let mut lines = Vec::new();

		for fragment in &extracted.fragments {
			let x = fragment.x;
			let y = fragment.y;
			let line = fragment.text.clone();

			let y_shift = (y - last_y).floor();

			if y_shift.abs() > 100.0 {
				blank = y_shift > 0.0; // End of Page Content
			}
			last_y = y;

			if blank || line.trim().chars().all(char::is_numeric) {
				continue;
			}

			if x > THRESHOLD {
				if x < r_indent.0 {
					r_indent.0 = x;
				}
			} else if x < l_indent.0 {
				l_indent.0 = x;
			}

			lines.push((x, line));
		}

		let mut last_indent = 0.0;
		let mut last_has_dot = false;
		let mut last_should_tab = false;
		let mut last_line = String::new();

		let lines: Vec<String> = lines
			.into_iter()
			.map(|(x, line)| {
				let min_x = if x < THRESHOLD {
					l_indent.0
				} else {
					r_indent.0
				};

				let dot = DOT_REGEX.is_match(&line);
				let indent = (x - min_x).floor();
				let indent = if dot { indent.max(9.0) } else { indent };

				let should_tab = if indent > last_indent {
					!(last_has_dot && !dot)
				} else if indent < last_indent {
					dot
				} else if last_line.trim().ends_with(':') && dot {
					true
				} else {
					last_should_tab
				};

				let indent = if indent == 0.0 && should_tab {
					9.0
				} else {
					indent
				};

				last_indent = indent;
				last_should_tab = should_tab;
				last_has_dot = dot;
				last_line.clone_from(&line);

				let prefix = if should_tab { "\t" } else { "" };

				if std::env::var("INDENT_DEBUG").is_ok() {
					format!("{indent}{prefix}{line}")
				} else {
					format!("{prefix}{line}")
				}
			})
			.collect();

		pages.insert(i as usize, lines);
	}

	Ok(pages)
}
