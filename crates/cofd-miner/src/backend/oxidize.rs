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
		preserve_layout: true,
		sort_by_position: true,
		reconstruct_paragraphs: true,
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

		let mut lines: Vec<(f64, String)> = Vec::new();

		for fragment in &extracted.fragments {
			let x = fragment.x;
			let y = fragment.y;
			let line = fragment.text.clone();

			let y_shift = (y - last_y).floor();

			// oxidize-pdf's fragment.y is in bottom-up page coordinates
			// (PDF native: y=0 at bottom of page, y=792 at top). The
			// original mupdf port used `line.bounds().y0` which is in
			// top-down coordinates (y=0 at top of page, increasing as
			// you go down). The "End of Page Content" check therefore
			// needs to fire when y goes *down* past the content
			// (negative y_shift in bottom-up), not when it goes *up*
			// (positive y_shift in top-down). The first fragment of a
			// new page has a large positive y_shift (jumping from
			// last_y=0.0 to y≈739 at the top of the page); inverting
			// the check skips the bogus "blank" flag that was
			// previously dropping every page after page 1.
			if y_shift.abs() > 100.0 {
				blank = y_shift < 0.0; // End of Page Content (bottom-up y)
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

			// Group adjacent fragments on the same baseline into a single line.
			// oxidize-pdf emits one TextFragment per Tj operator; mupdf's
			// TextPage already concatenates per-Tj fragments on the same
			// baseline into a single TextLine, so to keep the per-line
			// indent-tab state machine working we must do the same here.
			// Tolerance: 2.0 PDF units handles sub-pixel y jitter from
			// descenders on adjacent lines.
			//
			// Concatenation: PDF Tj operators typically render adjacent
			// glyphs in the same text run, so the next Tj starts where the
			// previous one ended. We concatenate without inserting a
			// space; if a wider gap exists (a true inter-word break), the
			// source PDF usually emits a space character inside the Tj's
			// string operand, which already preserves itself here.
			if let Some((prev_x, prev_text)) = lines.last_mut()
				&& y_shift.abs() < 2.0
			{
				if x < *prev_x {
					*prev_x = x;
				}
				prev_text.push_str(&line);
				continue;
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
					!last_has_dot || dot
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
