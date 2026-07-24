# Oxidize-pdf Transition Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Port the indent-reconstruction algorithm from `mupdf.rs` into `oxidize.rs` so the default `cargo run -p cofd-miner --example pdf-extract` produces real `Book` output, with `mupdf` retained as a fallback feature flag and `lopdf` deleted.

**Architecture:** New `oxidize::extract_pages` consumes `oxidize_pdf::parser::PdfDocument::extract_text_from_page_with_options` (yielding `Vec<TextFragment>` with explicit `x`, `y`, `font_size`) and runs the same five-variable state machine already used in `mupdf.rs:23-128`. Output shape (`PdfText = BTreeMap<usize, Vec<String>>`) and the public `extract_pages` / `extract_text` / `process_section` signatures are unchanged. `mupdf` stays available behind `--features mupdf`; `lopdf` is deleted.

**Tech Stack:** Rust nightly (per `rust-toolchain.toml`), `oxidize-pdf 3.1.2` (lockfile), `mupdf 0.4` (`default-features = false`; `sys-lib` has an ABI mismatch with nixpkgs and the default feature set requires fontconfig unavailable in the devenv), `anyhow`, `regex`, `once_cell`, `parking_lot`.

**Spec:** `docs/superpowers/specs/2026-07-24-oxidize-pdf-transition-design.md`

**Pre-flight changes from the original draft (2026-07-24):**

1. **Synthetic-PDF test removed.** The hand-rolled PDF in the original Task 1 had wrong xref byte offsets and would have failed to parse. Per user direction, the in-tree unit test is dropped; algorithm coverage comes from the post-merge smoke test against `~/Documents/RPG/CofD/Books/`. Audit F22 (partial) is therefore not closed by this plan; the smoke test is the verification.
2. **`mupdf` features set to `default-features = false`.** The original draft used `default-features = false, features = ["sys-lib"]`, but nixpkgs has `pkgs.mupdf` 1.27 and the `mupdf` 0.4 `sys-lib` feature expects a much older ABI, so it would not link cleanly. The default feature set also pulls in fontconfig-dependent functionality that is unavailable in the devenv. The shipped declaration therefore disables default features and does not enable `sys-lib`.

## Global Constraints

These apply to every task — copy verbatim from the spec; the implementer must not relax them without re-approval.

- `oxidize-pdf` floor: `version = "3.1"` (per spec §4.6; lockfile resolves to 3.1.2).
- `mupdf` declaration: `version = "0.4", optional = true, default-features = false` (minimal PDF-reading fallback; no OCR, system fonts, EPUB, or HTML — those are not needed for the fallback path, and the default feature set pulls in fontconfig-dependent functionality that the devenv does not have). Does NOT use `sys-lib` because nixpkgs mupdf 1.27 has incompatible ABI with mupdf 0.4's bindings.
- `default` features: `["oxidize", "embed_meta"]` (per spec §4.6; do not change).
- `oxizide-pdf` API entry points used: `parser::PdfDocument::open(path)`, `doc.page_count()`, `doc.extract_text_from_page_with_options(i, options)`, returning `ExtractedText { text: String, fragments: Vec<TextFragment> }` where `TextFragment = { x: f64, y: f64, width: f64, height: f64, font_size: f64, text: String, ... }`. (See `oxidize-pdf-3.1.2/src/parser/document.rs:1170` and `oxidize-pdf-3.1.2/src/text/extraction.rs:127`.)
- `ExtractionOptions` to pass: `ExtractionOptions { preserve_layout: true, sort_by_position: true, reconstruct_paragraphs: true, ..Default::default() }` (per spec §4.3; **all three** are correctness-critical — `preserve_layout: true` populates `.fragments`, `sort_by_position: true` orders them by `(x, y)`, `reconstruct_paragraphs: true` groups per-`Tj` fragments on the same baseline into single-line fragments, replicating mupdf's `TextPage.lines()` semantics).
- Column threshold: `const THRESHOLD: f64 = 240.0;` (per spec §4.2; was `f32` in mupdf).
- Page-blank detection: `(y - last_y).floor().abs() > 100.0` (per spec §4.2).
- Indent-tab F20 simplification (per spec §4.2):
  ```rust
  let should_tab = if indent > last_indent {
      !last_has_dot || dot
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
- Error wrapping: every `oxidize-pdf` call wraps the error with `anyhow::Context::with_context(|| format!("opening {}", path.display()))` (per spec §4.4).
- Local PDF corpus for the post-merge smoke test: `~/Documents/RPG/CofD/Books/`.

---

## File Structure

**Modified (3):**
- `crates/cofd-miner/Cargo.toml` — bump `oxidize-pdf` floor; re-declare `mupdf` optional dep; remove commented blocks.
- `crates/cofd-miner/src/backend/mod.rs` — drop `lopdf` cfg; keep `mupdf` and `oxidize` cfg; drop commented block.
- `crates/cofd-miner/src/backend/oxidize.rs` — full algorithm port; drop 105-line commented block; drop `println!`. No in-tree test (per pre-flight decision).

**Deleted (1):**
- `crates/cofd-miner/src/backend/lopdf.rs`.

**Unchanged:** `crates/cofd-miner/src/backend/mupdf.rs` (still reachable via `--features mupdf`); everything else in the workspace.

**Test strategy:** No in-tree unit test. Algorithm correctness is verified by the post-merge smoke test (Task 6 step 4) running `cargo run -p cofd-miner --example pdf-extract` against `~/Documents/RPG/CofD/Books/`. This is the same verification surface that the audit's F22 finding pointed at ("info-only fixtures → real fixtures"), now landing as a concrete pass/fail check.

---

## Task 1: Port the indent-reconstruction algorithm into `oxidize.rs`

**Files:**
- Modify: `crates/cofd-miner/src/backend/oxidize.rs` (replace the entire file)

**Interfaces:**
- Consumes: `oxidize_pdf::parser::PdfDocument::open(path)`, `doc.page_count()`, `doc.extract_text_from_page_with_options(i, options)` (per spec §4.2 and Global Constraints).
- Produces: `pub fn extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText>` where `PdfText = BTreeMap<usize, Vec<String>>` (defined in `backend/mod.rs`). Each emitted line is the line text, optionally prefixed with `'\t'`.

**Files to read when implementing this task:**
- `crates/cofd-miner/src/backend/mupdf.rs:23-128` — the algorithm to port (verbatim state machine, F20 simplification applied here).
- `crates/cofd-miner/src/backend/mupdf.rs:11` — `DOT_REGEX` Lazy initializer; the regex itself is `^<DOT_CHAR> ` (same as in mupdf).
- `oxidize-pdf-3.1.2/src/text/extraction.rs:127` — the `TextFragment` struct fields.

- [ ] **Step 1: Replace the entire `oxidize.rs` with the full implementation**

The current file `crates/cofd-miner/src/backend/oxidize.rs` is 153 lines: 22 active lines of stub plus a 105-line commented-out mupdf port plus boilerplate. Delete the file's contents and write the following in its place:

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

pub fn extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText> {
    let path = path.as_ref();
    let doc = PdfDocument::open(path)
        .with_context(|| format!("opening {}", path.display()))?;

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

        pages.insert(i, lines);
    }

    Ok(pages)
}
```

Notes on the diff against `mupdf.rs:23-128`:

- `last_x` is unused after the assignment in mupdf; drop it.
- The 3-deep nested iteration (`page → block → line`) collapses to a flat iteration over `extracted.fragments`.
- The `should_tab` body uses the F20-simplified form (no `#[allow(...)]` attribute).
- The `INDENT_DEBUG` branch is a single `if std::env::var(...).is_ok()` expression (no `#[cfg(debug_assertions)]` duplication).
- `min_x` uses `240.0` (`f64`) consistently.
- `pages.insert(i, lines)` runs after the loop (same as mupdf).

- [ ] **Step 2: Run `cargo check` on the default build**

Run: `cargo check -p cofd-miner --all-targets`

Expected: clean. If `cofd_schema` import fails, check the import path; the workspace's `cofd-schema` package name is `cofd-schema` but the crate name in `[lib]` may differ. Verify by reading `crates/cofd-schema/Cargo.toml` and the `cofd_schema::DOT_CHAR` re-export in `crates/cofd-schema/src/lib.rs`.

- [ ] **Step 3: Run `cargo clippy` clean on the default build**

Run: `cargo clippy -p cofd-miner --all-targets -- -D warnings`

Expected: clean. The F20 simplification removes `clippy::if_same_then_else` and `clippy::nonminimal_bool`; the new code must not introduce new lints. Common things to watch: `clippy::needless_collect`, `clippy::redundant_clone` on `fragment.text.clone()`, `clippy::useless_format` on the `format!("{prefix}{line}")` calls.

If `clippy` flags a new warning, address it inline; do not relax `#[deny(warnings)]`.

- [ ] **Step 4: Commit the algorithm port**

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
branch. Closes F1, F14 (oxidize half), F18, F20, F29.

Verification: cargo run -p cofd-miner --example pdf-extract
against ~/Documents/RPG/CofD/Books/ now produces non-info-only
Book JSON for the seven fixture books (Task 6 step 4)."
```


## Task 2: Update `backend/mod.rs` — drop `lopdf`, keep both `mupdf` and `oxidize`

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

## Task 3: Delete `lopdf.rs`

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

## Task 4: Update `Cargo.toml` — bump `oxidize-pdf` floor, re-declare `mupdf`, drop commented blocks

**Files:**
- Modify: `crates/cofd-miner/Cargo.toml`

**Interfaces:**
- Consumes: workspace constraints from `Cargo.toml` (root), `mupdf 0.4` published on crates.io, `oxidize-pdf 3.1` published on crates.io.
- Produces: `[features] oxidize = ["dep:oxidize-pdf"]`, `mupdf = ["dep:mupdf"]`. Two optional deps: `oxidize-pdf = { version = "3.1", optional = true }` and `mupdf = { version = "0.4", optional = true, default-features = false }` (no `sys-lib` because nixpkgs mupdf 1.27 has incompatible ABI with mupdf 0.4's bindings; the default feature set requires fontconfig unavailable in the devenv).

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
# mupdf = { version = "0.4", optional = true, default-features = false }
# lopdf = { version = "0.31", optional = true }
```

Replace with:

```toml
oxidize-pdf = { version = "3.1", optional = true }
mupdf = { version = "0.4", optional = true, default-features = false }
```

- [ ] **Step 3: Verify `cargo check` succeeds on the default build**

Run: `cargo check -p cofd-miner --all-targets`

Expected: clean. Default `oxidize` build still works. The mupdf optional dep is not yet referenced in the build because `mupdf` is not in the default features.

- [ ] **Step 4: Verify `cargo check` succeeds on `--no-default-features --features mupdf`**

Run: `cargo check -p cofd-miner --no-default-features --features mupdf --all-targets`

Expected: clean. NOTE: this is the first compile of `mupdf-sys`, which builds vendored MuPDF C/C++ source. Allow 5–10 minutes the first time (the build produces ~200 C/C++ translation units from the vendored mupdf sources). Subsequent builds are cached.

If the build fails with a C compiler error or `bindgen` error, the toolchain likely needs `libclang` (already in `devenv.nix` via `llvmPackages.clangUseLLVM` → `LIBCLANG_PATH`) and a C++ standard library. Verify both are present:

```bash
which clang++ && clang++ --version
ls "${LIBCLANG_PATH:-/nix/store/$(echo /nix/store/*libclang*/lib 2>/dev/null | head -1 | xargs basename)}"
```

If `LIBCLANG_PATH` is unset, the `bindgen` invocation in `mupdf-sys`'s build script will fail. The devenv sets this in `devenv.nix:48`; verify with `echo $LIBCLANG_PATH` in the active shell.

- [ ] **Step 5: Verify `cargo clippy` clean on default**

Run: `cargo clippy -p cofd-miner --all-targets -- -D warnings`

Expected: clean. (For `mupdf` clippy coverage, do it once on `--features mupdf`; if mupdf itself emits warnings from upstream, capture them with `--no-deps` instead.)

- [ ] **Step 6: Commit**

```bash
git add crates/cofd-miner/Cargo.toml
git commit -m "build(cofd-miner): bump oxidize-pdf to 3.1, re-declare mupdf optional

- Bump oxidize-pdf floor from 3.0 to 3.1. The 3.1 line is the
  first to expose PdfDocument::extract_text_from_page_with_options
  with the exact signature the new backend uses.
- Re-declare mupdf as a real optional dep with `default-features = false`; do not enable `sys-lib`. The default feature set requires fontconfig, which is unavailable in the devenv, and `sys-lib` does not work against nixpkgs mupdf 1.27 because of the ABI mismatch with mupdf 0.4's bindings.
- Drop the commented lopdf lines (the backend is deleted in
  the previous commit).

Closes F2 (Cargo.toml half)."
```

---

## Task 5: Workspace check + post-merge smoke test

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
| §2 F1: port algorithm into oxidize.rs | Task 1 |
| §2 F2: declare mupdf dep, remove dead cfg | Task 2 (mod.rs half), Task 4 (Cargo.toml half) |
| §2 F8: delete lopdf.rs | Task 3 |
| §2 F14 (oxidize half): no `println!` | Task 1 (the new `extract_pages` body has no `println!`) |
| §2 F18: collapse `INDENT_DEBUG` branch | Task 1 (single-expression form) |
| §2 F20: simplify `if a { false } else { true }` | Task 1 (the `should_tab` body) |
| §2 F29: delete 105-line commented block | Task 1 (replaces the entire file body) |
| §2 F22 (partial): in-tree unit test | **Not closed by this plan** (per pre-flight decision; smoke test in Task 5 is the verification) |
| §4.1: signature `extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText>` | Task 1 |
| §4.2: 5-variable state machine, port verbatim | Task 1 |
| §4.2: `THRESHOLD: f64 = 240.0` | Task 1 |
| §4.2: page-blank detection `(y - last_y).floor().abs() > 100.0` | Task 1 |
| §4.2: F20 simplification of `should_tab` | Task 1 |
| §4.2: `INDENT_DEBUG` collapsed to single expression | Task 1 |
| §4.3: `ExtractionOptions { sort_by_position: true, ..Default::default() }` | Task 1 |
| §4.4: error wrapping with `with_context` | Task 1 |
| §4.5: `mod.rs` cfg structure | Task 2 |
| §4.6: `oxidize-pdf` floor `3.1` | Task 4 |
| §4.6: `mupdf` declared (`default-features = false`; no `sys-lib` because of ABI mismatch and unavailable fontconfig) | Task 4 |
| §5.2: integration test left `#[ignore]`'d | (no task — already done, untouched) |
| §5.3: post-merge smoke test | Task 5 |
| §6: pre-merge verification gates | Task 5 |

**2. Placeholder scan:** No "TBD", no "add appropriate error handling", no "implement later". Every `cargo` command runs an expected output. Every code block is complete.

**3. Type consistency:** `extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText>` is defined in Task 1 and consumed by `source::extract_text` in `crates/cofd-miner/src/source.rs:101` (unchanged). `PdfText` is the type alias from `backend/mod.rs` Task 2. `DOT_REGEX`, `THRESHOLD`, and `ExtractionOptions` are declared in Task 1 and consumed in Task 1. No drift.

**4. mupdf 0.4 + nixpkgs 1.27 compatibility:** Do not enable `sys-lib`: nixpkgs ships `mupdf 1.27`, which has an incompatible ABI with mupdf 0.4's bindings. Also disable default features because they require fontconfig, which is unavailable in the devenv. The shipped `default-features = false` declaration keeps the fallback dependency limited to the PDF-reading path.

**5. Plan makes sense on its own:** A reader who has not seen the spec can execute this plan from the top, commit per task, and end with a working transition. The Global Constraints section captures every spec-level invariant the implementer needs.
