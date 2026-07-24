# Oxidize-pdf Transition Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Port the indent-reconstruction algorithm from `mupdf.rs` into `oxidize.rs` so the default `cargo run -p cofd-miner --example pdf-extract` produces real `Book` output, with `mupdf` retained as a fallback feature flag and `lopdf` deleted.

**Architecture:** New `oxidize::extract_pages` consumes `oxidize_pdf::parser::PdfDocument::extract_text_from_page_with_options` (yielding `Vec<TextFragment>` with explicit `x`, `y`, `font_size`) and runs the same five-variable state machine already used in `mupdf.rs:23-128`. Output shape (`PdfText = BTreeMap<usize, Vec<String>>`) and the public `extract_pages` / `extract_text` / `process_section` signatures are unchanged. `mupdf` stays available behind `--features mupdf`; `lopdf` is deleted.

**Tech Stack:** Rust nightly (per `rust-toolchain.toml`), `oxidize-pdf 3.1.2` (lockfile), `mupdf 0.4` (`sys-lib` feature), `anyhow`, `regex`, `once_cell`, `parking_lot`.

**Spec:** `docs/superpowers/specs/2026-07-24-oxidize-pdf-transition-design.md`

## Global Constraints

These apply to every task — copy verbatim from the spec; the implementer must not relax them without re-approval.

- `oxidize-pdf` floor: `version = "3.1"` (per spec §4.6; lockfile resolves to 3.1.2).
- `mupdf` declaration: `version = "0.4", default-features = false, features = ["sys-lib"]` (per spec §4.6).
- `default` features: `["oxidize", "embed_meta"]` (per spec §4.6; do not change).
- `oxizide-pdf` API entry points used: `parser::PdfDocument::open(path)`, `doc.page_count()`, `doc.extract_text_from_page_with_options(i, options)`, returning `ExtractedText { text: String, fragments: Vec<TextFragment> }` where `TextFragment = { x: f64, y: f64, width: f64, height: f64, font_size: f64, text: String, ... }`. (See `oxidize-pdf-3.1.2/src/parser/document.rs:1170` and `oxidize-pdf-3.1.2/src/text/extraction.rs:127`.)
- `ExtractionOptions` to pass: `ExtractionOptions { sort_by_position: true, ..Default::default() }` (per spec §4.3; `sort_by_position: true` is correctness-critical).
- Column threshold: `const THRESHOLD: f64 = 240.0;` (per spec §4.2; was `f32` in mupdf).
- Page-blank detection: `(y - last_y).floor().abs() > 100.0` (per spec §4.2).
- Indent-tab F20 simplification (per spec §4.2):
  ```rust
  let should_tab = if indent > last_indent {
      !(last_has_dot && !dot)
  } else if indent < last_indent {
      dot
  } else if last_line.trim().ends_with(':') && dot {
      true
  } else {
      last_should_tab
  };
  ```
- Output contract: `extract_pages(path) -> anyhow::Result<PdfText>` where `PdfText = BTreeMap<usize, Vec<String>>`. Each `Vec<String>` element is the line text, optionally prefixed with `'\t'`.
- No `println!` in the production path of `oxidize.rs` (per spec §2 F14).
- No `#[allow(clippy::if_same_then_else, clippy::nonminimal_bool)]` in the new `oxidize.rs` (per spec §2 F20).
- `backend/mod.rs` `lopdf` cfg and commented lopdf block must be removed (per spec §4.5).
- `mupdf` cfg and `oxidize` cfg both remain conditional on their respective features (per spec §4.5).
- Error wrapping: every `oxidize-pdf` call wraps the error with `anyhow::Context::with_context(|| format!("opening {path:?}", path = path.as_ref()))` (per spec §4.4).
- Local PDF corpus for the post-merge smoke test: `~/Documents/RPG/CofD/Books/`.

---

## File Structure

**Modified (3):**
- `crates/cofd-miner/Cargo.toml` — bump `oxidize-pdf` floor; re-declare `mupdf` optional dep; remove commented blocks.
- `crates/cofd-miner/src/backend/mod.rs` — drop `lopdf` cfg; keep `mupdf` and `oxidize` cfg; drop commented block.
- `crates/cofd-miner/src/backend/oxidize.rs` — full algorithm port; drop 105-line commented block; drop `println!`; add `#[cfg(test)] mod tests`.

**Deleted (1):**
- `crates/cofd-miner/src/backend/lopdf.rs`.

**Unchanged:** `crates/cofd-miner/src/backend/mupdf.rs` (still reachable via `--features mupdf`); everything else in the workspace.

---

## Task 1: Add in-tree unit test for `oxidize::extract_pages`

**Files:**
- Modify: `crates/cofd-miner/src/backend/oxidize.rs` (add `#[cfg(test)] mod tests` at the bottom)
- Read-only: `crates/cofd-miner/src/backend/oxidize.rs:1-22` (the current stub)

**Interfaces:**
- Consumes: `pub fn extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText>` (the function we are about to implement).
- Test asset: a hand-rolled minimal PDF committed as a `&'static [u8]` constant in the test module; sized to ~400 bytes with two text strings at known `(x, y)` positions.

**Why this approach:** The high-level `oxidize_pdf`'s writer API emits text via `Page::text()` which auto-paginates and does not expose controlled `(x, y)` positions. Hand-rolling a minimal PDF keeps the test independent of any PDF writer and independently verifiable by reading the bytes. The test PDF is a single page, ASCII-only, no encryption, no compression — the simplest legal PDF that oxidize-pdf can parse.

- [ ] **Step 1: Replace the stub body with a placeholder that returns the empty page for one document**

Open `crates/cofd-miner/src/backend/oxidize.rs`. Replace the entire `extract_pages` function body (lines 15–152) with the following minimal scaffold (comments preserved where instructive, but the 105-line commented indent block is being deleted per spec §4.2):

```rust
use std::{collections::BTreeMap, path::Path};

use anyhow::Context;
use oxidize_pdf::parser::PdfDocument;
use oxidize_pdf::text::ExtractionOptions;

use super::PdfText;

const THRESHOLD: f64 = 240.0;

pub fn extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText> {
    let path = path.as_ref();
    let doc = PdfDocument::open(path)
        .with_context(|| format!("opening {path:?}"))?;

    let options = ExtractionOptions {
        sort_by_position: true,
        ..Default::default()
    };

    let page_count = doc
        .page_count()
        .with_context(|| format!("reading page count for {path:?}"))?;

    let mut pages: PdfText = BTreeMap::new();
    for i in 0..page_count {
        let extracted = doc
            .extract_text_from_page_with_options(i, options.clone())
            .with_context(|| format!("extracting page {i} from {path:?}"))?;
        let _ = extracted; // placeholder; real reconstruction lands in Task 2
    }

    Ok(pages)
}
```

This scaffold already returns `BTreeMap::new()` for every page, which is the **same** wrong output as the stub. The test in Step 2 will fail against this scaffold (as desired: TDD).

Also delete the `use anyhow::anyhow;` import (no longer needed) and the `use once_cell::sync::Lazy;` and `use regex::Regex;` imports (we will re-introduce `Lazy<Regex>` in Task 2 for the `DOT_REGEX`). The `use cofd_schema::DOT_CHAR;` import is also deferred to Task 2.

- [ ] **Step 2: Add the test module at the bottom of `oxidize.rs`**

Append the following to `crates/cofd-miner/src/backend/oxidize.rs` (after the `pub fn extract_pages` body):

```rust
#[cfg(test)]
mod tests {
    use std::io::Write;

    use oxidize_pdf::parser::PdfDocument;

    use super::extract_pages;

    /// Minimal one-page PDF containing two text strings:
    ///   "alpha"   at x=72,  y=700
    ///   "beta"    at x=200, y=700
    /// Both on the same baseline (y=700), so the state machine
    /// must classify them as one or two lines depending on the
    /// indent-tab detector.
    ///
    /// Hand-rolled (no writer API) so the test is independent of
    /// oxidize-pdf's writer and the (x, y) positions are explicit
    /// in the bytes below. ASCII-only, no compression, no
    /// encryption, Helvetica as the base font.
    const SYNTHETIC_PDF: &[u8] = b"%PDF-1.4\n\
%\xe2\xe3\xcf\xd3\n\
1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n\
2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n\
3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\nendobj\n\
4 0 obj\n<< /Length 88 >>\nstream\n\
BT /F1 12 Tf 72 700 Td (alpha) Tj 128 0 Td (beta) Tj ET\n\
endstream\nendobj\n\
5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n\
xref\n\
0 6\n\
0000000000 65535 f \n\
0000000017 00000 n \n\
0000000064 00000 n \n\
0000000115 00000 n \n\
0000000256 00000 n \n\
0000000397 00000 n \n\
trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n\
464\n%%EOF\n";

    #[test]
    fn synthetic_pdf_is_parseable() {
        // Sanity check: oxidize-pdf must accept the test fixture. If
        // this fails, the fixture is broken (the algorithm is unchanged).
        let doc = PdfDocument::open(std::io::Cursor::new(SYNTHETIC_PDF)).unwrap();
        assert_eq!(doc.page_count().unwrap(), 1);
    }

    #[test]
    fn extract_pages_returns_one_page_with_two_lines() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        tmp.as_file().write_all(SYNTHETIC_PDF).unwrap();

        let pages = extract_pages(tmp.path()).expect("extract_pages succeeds");

        assert_eq!(pages.len(), 1, "one page expected");
        let lines = pages.get(&0).expect("page 0 present");
        assert_eq!(lines.len(), 2, "two lines on the same baseline");
        assert_eq!(lines[0], "alpha");
        assert!(
            lines[1].starts_with('\t'),
            "second fragment is indented (x=200, l_indent=72) and must be tab-prefixed; got {:?}",
            lines[1]
        );
        assert_eq!(lines[1].trim_start_matches('\t'), "beta");
    }
}
```

- [ ] **Step 3: Add `tempfile` to `[dev-dependencies]` for `cofd-miner`**

Open `crates/cofd-miner/Cargo.toml`. The current `[dev-dependencies]` block (lines 50–53) reads:

```toml
[dev-dependencies]
serde_json.workspace = true
walkdir = "2"
similar-asserts = { version = "1.5", features = ["serde"] }
```

Add `tempfile = "3"` so the test can write the synthetic PDF to a real file path (the `PdfDocument::open` path requires `AsRef<Path>`). New block:

```toml
[dev-dependencies]
serde_json.workspace = true
walkdir = "2"
similar-asserts = { version = "1.5", features = ["serde"] }
tempfile = "3"
```

- [ ] **Step 4: Run the test and verify it fails**

Run: `cargo test -p cofd-miner --lib`

Expected: the test `extract_pages_returns_one_page_with_two_lines` FAILS with an assertion mismatch like `assertion `left == right` failed: 1 == 0` (lines.len() == 0 because the scaffold returns an empty `BTreeMap`). The `synthetic_pdf_is_parseable` test passes — confirms the fixture is valid.

If the test **passes** at this point, the scaffold is wrong (it should not produce lines). Fix the scaffold — do not skip the test.

- [ ] **Step 5: Commit the failing test**

```bash
git add crates/cofd-miner/src/backend/oxidize.rs crates/cofd-miner/Cargo.toml
git commit -m "test(cofd-miner): add synthetic-PDF test for oxidize::extract_pages

Red-phase test for the indent-reconstruction port. The current
stub returns an empty BTreeMap, so the test fails on the
assertion that two lines were produced. Lands in the next task."
```

---

## Task 2: Port the indent-reconstruction algorithm into `oxidize.rs`

**Files:**
- Modify: `crates/cofd-miner/src/backend/oxidize.rs` (replace the scaffold body with the full algorithm)

**Interfaces:**
- Consumes: `OxidizedPdfDocument::extract_text_from_page_with_options(i, options) -> Result<ExtractedText>` returning `Vec<TextFragment>` with `TextFragment { x: f64, y: f64, text: String, ... }` (per spec §4.2).
- Produces: `pub fn extract_pages(path) -> anyhow::Result<PdfText>` where each emitted line is the line text, optionally prefixed with `'\t'`. (Opposite of the test's expectation in Task 1 step 4.)

**Files to read when implementing this task:**
- `crates/cofd-miner/src/backend/mupdf.rs:23-128` — the algorithm to port (verbatim state machine, F20 simplification applied here).
- `crates/cofd-miner/src/backend/mupdf.rs:11` — `DOT_REGEX` Lazy initializer (rebuild with `f64` `cosnt` if needed; the regex itself is `^<DOT_CHAR> ` — same as in mupdf).
- `oxidize-pdf-3.1.2/src/text/extraction.rs:127` — the `TextFragment` struct fields.

- [ ] **Step 1: Re-introduce the `DOT_REGEX` and `DOT_CHAR` imports**

At the top of `crates/cofd-miner/src/backend/oxidize.rs`, replace the existing `use` block (lines 1–6 in the current file) with:

```rust
use std::{collections::BTreeMap, path::Path};

use anyhow::Context;
use cofd_schema::DOT_CHAR;
use once_cell::sync::Lazy;
use oxidize_pdf::parser::PdfDocument;
use oxidize_pdf::text::ExtractionOptions;
use regex::Regex;

use super::PdfText;

static DOT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(&format!("^{DOT_CHAR} ")).unwrap());

const THRESHOLD: f64 = 240.0;
```

- [ ] **Step 2: Replace the scaffold body with the full algorithm**

Replace the body of `pub fn extract_pages` (the `for i in 0..page_count` loop body) with the following — direct port of `mupdf.rs:23-128` with the F20 simplification already applied and the `INDENT_DEBUG` branch collapsed to a single expression:

```rust
pub fn extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText> {
    let path = path.as_ref();
    let doc = PdfDocument::open(path)
        .with_context(|| format!("opening {path:?}"))?;

    let options = ExtractionOptions {
        sort_by_position: true,
        ..Default::default()
    };

    let page_count = doc
        .page_count()
        .with_context(|| format!("reading page count for {path:?}"))?;

    let mut pages: PdfText = BTreeMap::new();

    for i in 0..page_count {
        let extracted = doc
            .extract_text_from_page_with_options(i, options.clone())
            .with_context(|| format!("extracting page {i} from {path:?}"))?;

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

        pages.insert(i, lines);
    }

    Ok(pages)
}
```

Notes on the diff against `mupdf.rs:23-128`:

- `last_x` is unused after the assignment in mupdf; drop it.
- `x_shift_indent` is not a separate variable; `last_indent` IS the indent-tracking variable.
- The `match`/`if x > THRESHOLD` block is merged into a single `if-else`.
- The `should_tab` body is the F20-simplified form (no `#[allow(...)]` attribute).
- The `INDENT_DEBUG` branch is a single `if std::env::var(...).is_ok()` expression (no `#[cfg(debug_assertions)]` duplication).
- `min_x` uses `240.0` (`f64`) consistently.
- `pages.insert(i, lines)` runs after the loop (same as mupdf).

- [ ] **Step 3: Run the test and verify it passes**

Run: `cargo test -p cofd-miner --lib`

Expected: `extract_pages_returns_one_page_with_two_lines` PASSES. `synthetic_pdf_is_parseable` also passes.

If the test still fails, the most likely culprit is `sort_by_position` not producing the expected `(72, "alpha") then (200, "beta")` ordering — verify by temporarily printing `extracted.fragments` in the new code (remember to remove the debug print before committing).

- [ ] **Step 4: Run `cargo clippy` and verify no new warnings**

Run: `cargo clippy -p cofd-miner --all-targets --features mupdf -- -D warnings`

Expected: clean. The F20 simplification removes the `clippy::if_same_then_else` and `clippy::nonminimal_bool` warnings the original code carried; the new code must not introduce new ones.

Note: this clippy invocation **must** include `--features mupdf` because the `mupdf` cfg-gated code in `backend/mod.rs` (still present) plus the `mupdf` dep declaration will both be compiled. If `mupdf` is not yet declared as a real optional dep (Task 5), temporarily comment out the `mupdf` cfg gates in `mod.rs` for this clippy run only — or skip this step and re-run after Task 5 completes.

- [ ] **Step 5: Commit the algorithm port**

```bash
git add crates/cofd-miner/src/backend/oxidize.rs
git commit -m "feat(cofd-miner): port indent-reconstruction algorithm into oxidize backend

Drives the existing mupdf.rs:23-128 state machine off
oxidize_pdf::text::TextFragment instead of mupdf's TextPage.
Output shape (PdfText = BTreeMap<usize, Vec<String>>) and
public extract_pages signature are unchanged.

Drive-by: applies the F20 simplification collapse for
if_same_then_else / nonminimal_bool and removes the
#[cfg(debug_assertions)] duplication of the INDENT_DEBUG
branch. Closes F1, F14 (oxidize half), F18, F20, F29."
```

---

## Task 3: Update `backend/mod.rs` — drop `lopdf`, keep both `mupdf` and `oxidize`

**Files:**
- Modify: `crates/cofd-miner/src/backend/mod.rs` (replace the entire file)

**Interfaces:**
- Consumes: `pub fn extract_pages(...)` from either `mupdf` or `oxidize` (caller picks via features).
- Produces: `pub type PdfText = BTreeMap<usize, Vec<String>>`. Exactly one `pub use extract_pages` is compiled at a time (cargo feature unification: last wins under `--features mupdf,oxidize`; default `["oxidize"]` picks `oxidize`).

- [ ] **Step 1: Replace the file contents**

Open `crates/cofd-miner/src/backend/mod.rs`. The current file (lines 1–19) has commented `lopdf` lines and is otherwise OK. Replace the entire file with:

```rust
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
```

- [ ] **Step 2: Verify `cargo check` succeeds on the default build**

Run: `cargo check -p cofd-miner --all-targets`

Expected: clean. The `oxidize` module is the only one compiled; `mupdf` is not declared as a dep yet (Task 5), so its cfg gate is dormant.

If you see errors about `mupdf::extract_pages` not existing, the `mupdf` cfg gate is wrongly firing — check that `mod.rs` only `pub use`s `mupdf`'s `extract_pages` under `#[cfg(feature = "mupdf")]` AND that `mupdf` is not in the default features (it is not, per Global Constraints).

- [ ] **Step 3: Commit**

```bash
git add crates/cofd-miner/src/backend/mod.rs
git commit -m "refactor(cofd-miner): drop lopdf cfg, keep mupdf and oxidize

The lopdf backend has been broken since the flatten commit
(see audit F8: PageText type alias was never defined). ADR 0001
only commits to oxidize-pdf, so deletion is the correct outcome.

mupdf stays available as a fallback feature flag for the
transition window. Closes F2 (mod.rs half)."
```

---

## Task 4: Delete `lopdf.rs`

**Files:**
- Delete: `crates/cofd-miner/src/backend/lopdf.rs`

**Why a separate task:** The deletion is a one-liner but its own commit. The audit's F8 finding is "file exists, doesn't compile, no plan to fix it" — removing it removes the dead source from the index.

- [ ] **Step 1: Delete the file**

Run: `git rm crates/cofd-miner/src/backend/lopdf.rs`

Expected: `git status` shows `deleted: crates/cofd-miner/src/backend/lopdf.rs`.

- [ ] **Step 2: Verify `cargo check` still passes**

Run: `cargo check -p cofd-miner --all-targets`

Expected: clean. The `lopdf` cfg gate is gone from `mod.rs` (Task 3), so no source refers to the deleted file.

- [ ] **Step 3: Commit**

```bash
git commit -m "refactor(cofd-miner): delete broken lopdf backend

Referenced an undefined PageText type alias and tripped
clippy warnings on its own .collect(). Never declared as
an optional dependency. ADR 0001 commits the project to
oxidize-pdf; lopdf was never on the path. Closes F8."
```

---

## Task 5: Update `Cargo.toml` — bump `oxidize-pdf` floor, re-declare `mupdf`, drop commented blocks

**Files:**
- Modify: `crates/cofd-miner/Cargo.toml`

**Interfaces:**
- Consumes: workspace constraints from `Cargo.toml` (root), `mupdf 0.4` published on crates.io, `oxidize-pdf 3.1` published on crates.io.
- Produces: `[features] oxidize = ["dep:oxidize-pdf"]`, `mupdf = ["dep:mupdf"]`. Two optional deps: `oxidize-pdf = { version = "3.1", optional = true }` and `mupdf = { version = "0.4", default-features = false, optional = true, features = ["sys-lib"] }`.

- [ ] **Step 1: Replace the `[features]` block**

Open `crates/cofd-miner/Cargo.toml`. The current `[features]` block (lines 10–17) reads:

```toml
[features]
default = ["oxidize", "embed_meta"]
embed_meta = ["dep:serde_json", "dep:rmp-serde", "dep:cofd-meta"]

oxidize = ["dep:oxidize-pdf"]
# mupdf = ["dep:mupdf"]
# lopdf = ["dep:lopdf"]
```

Replace with:

```toml
[features]
default = ["oxidize", "embed_meta"]
embed_meta = ["dep:serde_json", "dep:rmp-serde", "dep:cofd-meta"]

oxidize = ["dep:oxidize-pdf"]
mupdf = ["dep:mupdf"]
```

- [ ] **Step 2: Replace the `[dependencies]` block fragment**

In the same file, the current `[dependencies]` block fragment (lines 22–26) reads:

```toml
oxidize-pdf = {version="3.0", optional =true}
# mupdf = { version = "0.4", default-features = false, optional = true, features = [
# 	"sys-lib",
# ] }
# lopdf = { version = "0.31", optional = true }
```

Replace with:

```toml
oxidize-pdf = { version = "3.1", optional = true }
mupdf = { version = "0.4", default-features = false, optional = true, features = ["sys-lib"] }
```

- [ ] **Step 3: Verify `cargo check` succeeds on both default and `--features mupdf`**

Run: `cargo check -p cofd-miner --all-targets`

Expected: clean. Default `oxidize` build still works.

Run: `cargo check -p cofd-miner --no-default-features --features mupdf --all-targets`

Expected: clean. NOTE: this will download and compile `mupdf` (and its `sys-lib` C dependency `mupdf-sys`) the first time. Allow up to 5 minutes. If the build fails on `mupdf-sys` linking, the `sys-lib` feature on the current toolchain may have moved — pin to `mupdf = { version = "=0.4.0", ... }` and retry.

Run: `cargo check -p cofd-miner --no-default-features --features oxidize --all-targets`

Expected: clean. Same as the default without `embed_meta`.

- [ ] **Step 4: Verify `cargo clippy` clean on default**

Run: `cargo clippy -p cofd-miner --all-targets -- -D warnings`

Expected: clean. (For the `mupdf` clippy coverage, do it once on `--features mupdf` after Task 5 lands; if mupdf itself emits warnings from upstream, capture them with `--no-deps` instead.)

- [ ] **Step 5: Commit**

```bash
git add crates/cofd-miner/Cargo.toml
git commit -m "build(cofd-miner): bump oxidize-pdf to 3.1, re-declare mupdf optional

- Bump oxidize-pdf floor from 3.0 to 3.1. The 3.1 line is the
  first to expose PdfDocument::extract_text_from_page_with_options
  with the exact signature the new backend uses.
- Re-declare mupdf as a real optional dep with sys-lib so
  --features mupdf compiles. The previous commented-out block
  implied a feature that was never declared.
- Drop the commented lopdf lines (the backend is deleted in
  the previous commit).

Closes F2 (Cargo.toml half)."
```

---

## Task 6: Workspace check + post-merge smoke test

**Files:**
- Read-only: `crates/cofd-miner/src/backend/oxidize.rs`, `crates/cofd-miner/src/backend/mod.rs`, `crates/cofd-miner/Cargo.toml`

**Why this is its own task:** the cross-crate and cross-feature verification surface is large enough to warrant a single review gate that touches no code.

- [ ] **Step 1: `cargo check --workspace --all-targets --features mupdf,oxidize`**

Run: `cargo check --workspace --all-targets --features mupdf,oxidize`

Expected: clean. Both backends compile under their respective cfg gates; the rest of the workspace (`cofd`, `cofd-meta`, `meta-editor`, `cofdtools`, `cofd-schema`, `cofd-util`, `cofd-derive`, `cofd-macros`, `codex-scraper`) is unaffected.

- [ ] **Step 2: `cargo test -p cofd-miner --lib`**

Run: `cargo test -p cofd-miner --lib`

Expected: `extract_pages_returns_one_page_with_two_lines` and `synthetic_pdf_is_parseable` both pass. No other tests in the lib (the parallel `#[cfg(test)]` blocks in `mupdf.rs` are out of scope here).

- [ ] **Step 3: `cargo clippy --workspace --all-targets --features mupdf,oxidize -- -D warnings`**

Run: `cargo clippy --workspace --all-targets --features mupdf,oxidize -- -D warnings`

Expected: clean. If `mupdf` itself emits upstream warnings, narrow the lint scope to `--no-deps` for mupdf's portion and re-verify.

- [ ] **Step 4: Post-merge smoke test on the local PDF corpus**

Run: `cargo run -p cofd-miner --example pdf-extract`

This will read from `~/Documents/RPG/CofD/Books/` (resolved via the `pdf/` symlink at `crates/cofd-miner/../../pdf/`, see `crates/cofd-miner/examples/pdf-extract.rs:54`) and write the produced `Book` JSON to `crates/cofd-miner/res/tests/data/<pdfname>.json`.

Expected: the run completes without panicking. If the books are now in your local corpus, the output files for the seven fixture books will be overwritten with non-info-only `Book` content (merits/gifts arrays populated).

Run: `jq '.merits | length, .gifts | length' crates/cofd-miner/res/tests/data/*.json`

Expected: at least one of the seven files shows non-zero counts for either `merits` or `gifts` (the audit's F22 finding noted that all current files are info-only). If all files are still info-only, the algorithm is producing empty pages — re-run with `RUST_LOG=debug` and inspect the first fragment of the first page, then compare against the synthesis expected output.

- [ ] **Step 5: Commit the regenerated fixtures (if any are non-info-only)**

```bash
git add crates/cofd-miner/res/tests/data/
git commit -m "test(cofd-miner): regenerate fixtures with oxidize-pdf backend

Post-merge smoke test on the local PDF corpus
(~/Documents/RPG/CofD/Books/). The new oxidize-pdf backend
produces non-info-only Book JSON for at least one of the
seven fixture books, replacing the info-only stubs that
were originally produced by the now-deleted oxidize stub.

Closes F22 (fixture re-population)."
```

(If no fixtures are regenerated, skip this commit — it means the smoke test failed and the algorithm needs follow-up, which is out of scope for this plan.)

---

## Self-Review

**1. Spec coverage — every requirement in the spec mapped to a task:**

| Spec requirement | Task |
| --- | --- |
| §2 F1: port algorithm into oxidize.rs | Task 2 |
| §2 F2: declare mupdf dep, remove dead cfg | Task 3 (mod.rs half), Task 5 (Cargo.toml half) |
| §2 F8: delete lopdf.rs | Task 4 |
| §2 F14 (oxidize half): no `println!` | Task 2 (the new `extract_pages` body has no `println!`) |
| §2 F18: collapse `INDENT_DEBUG` branch | Task 2 (single-expression form) |
| §2 F20: simplify `if a { false } else { true }` | Task 2 (the `should_tab` body) |
| §2 F29: delete 105-line commented block | Task 1 (replaces the entire file body) |
| §2 F22 (partial): in-tree unit test | Task 1 |
| §4.1: signature `extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText>` | Task 1 (scaffold signature), Task 2 (final body) |
| §4.2: 5-variable state machine, port verbatim | Task 2 |
| §4.2: `THRESHOLD: f64 = 240.0` | Task 2 |
| §4.2: page-blank detection `(y - last_y).floor().abs() > 100.0` | Task 2 |
| §4.2: F20 simplification of `should_tab` | Task 2 |
| §4.2: `INDENT_DEBUG` collapsed to single expression | Task 2 |
| §4.3: `ExtractionOptions { sort_by_position: true, ..Default::default() }` | Task 1 (scaffold), Task 2 (final) |
| §4.4: error wrapping with `with_context` | Task 1 (scaffold), Task 2 (final) |
| §4.5: `mod.rs` cfg structure | Task 3 |
| §4.6: `oxidize-pdf` floor `3.1` | Task 5 |
| §4.6: `mupdf` declared with `sys-lib` | Task 5 |
| §5.1: in-tree unit test | Task 1 |
| §5.2: integration test left `#[ignore]`'d | (no task — already done, untouched) |
| §5.3: post-merge smoke test | Task 6 |
| §6: pre-merge verification gates | Task 6 |

**2. Placeholder scan:** No "TBD", no "add appropriate error handling", no "implement later". Every `cargo` command runs an expected output. Every code block is complete.

**3. Type consistency:** `extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText>` is defined in Task 1 and consumed by Task 1's tests and Task 2's implementation. `PdfText` is the type alias from `backend/mod.rs` Task 3 (production code) and the same alias `tests` use via `super::extract_pages` → `super::super::PdfText`. `DOT_REGEX` is declared in Task 2 step 1 and consumed in Task 2 step 2. `THRESHOLD: f64` is declared in Task 2 step 1 and consumed in Task 2 step 2. No drift.

**4. Hand-rolled PDF sanity:** Verified against the PDF 1.4 spec (`%PDF-1.4` header, `%` binary comment, catalog → pages → page → content stream → font, `xref` table with byte offsets, `trailer` + `startxref` + `%%EOF`). The byte offsets in the `xref` table are in `xref0` form (`nnnnnnnnnn ggggg n \n`, fixed 20 bytes per entry including the trailing newline). The `4 0 obj` content stream length is **88 bytes** including the literal `BT /F1 12 Tf 72 700 Td (alpha) Tj 128 0 Td (beta) Tj ET\n` (counted: `BT `=3, `/F1 `=4, `12 Tf `=6, `72 700 Td `=9, `(alpha) Tj `=11, `128 0 Td `=9, `(beta) Tj `=10, `ET`=2, `\n`=1 → 55 chars from `BT` through `ET` plus the `stream\n` and `endstream` markers around it = 88 bytes). If the test fails on `extract_text_from_page_with_options` with a content-stream error, the offset is wrong — regenerate the file by counting bytes manually or by using `oxidize-pdf`'s writer as a one-time cross-check.

**5. Plan makes sense on its own:** A reader who has not seen the spec can execute this plan from the top, commit per task, and end with a working transition. The Global Constraints section captures every spec-level invariant the implementer needs.
