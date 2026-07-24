# Spec: complete the transition to `oxidize-pdf`

**Date:** 2026-07-24
**Status:** approved
**Audit reference:** `docs/cofd-miner-audit-2026-07-24.md`

## 1. Problem

`cofd-miner` advertises itself as a PDF→structured-book extractor but the
default build extracts nothing. `crates/cofd-miner/src/backend/oxidize.rs:15-152`
is a 22-line stub that calls `oxidize_pdf::parser::PdfDocument::open` and
`doc.rag_chunks()?`, prints every chunk to stdout via `println!`, and returns
an empty `BTreeMap::new()`. The only fully-implemented backend
(`crates/cofd-miner/src/backend/mupdf.rs:15-134`) is feature-gated off and its
underlying `mupdf` crate is not even declared as an optional dependency in
`crates/cofd-miner/Cargo.toml:18-26`, so `--features mupdf` cannot compile
cleanly. The `lopdf` backend (`crates/cofd-miner/src/backend/lopdf.rs:7-27`)
references an undefined `PageText` type alias. ADR 0001
(`docs/adr/0001-monorepo-architecture.md:149-152`) commits the project to
`oxidize-pdf 3.0` as the active default; the code does not fulfill that
commitment.

This plan finishes that transition: ports the indent-reconstruction algorithm
from `mupdf.rs` into the `oxidize.rs` backend so the default build produces
real output, retains `mupdf` as a fallback feature flag during the transition
window, and deletes the unfixable `lopdf` code.

## 2. Scope

**In scope** (audit references in parens):

- F1: port the indent-reconstruction algorithm from `mupdf.rs` into
  `oxidize.rs`; the default `oxidize` backend produces real `PdfText`.
- F2: declare `mupdf` as a real optional dependency in `Cargo.toml` (not
  commented out) so `--features mupdf` actually compiles; remove the dead
  `#[cfg(feature = "lopdf")]` and commented `mupdf` lines in
  `backend/mod.rs` and `Cargo.toml`.
- F8: delete `crates/cofd-miner/src/backend/lopdf.rs` (the `PageText` typo
  means it cannot compile; no fix needed since ADR 0001 only commits to
  `oxidize-pdf`).
- F14 (oxidize.rs half): remove the `println!` in the new active `oxidize.rs`
  production path.
- F18: while porting the indent logic, collapse the `INDENT_DEBUG` env-var
  branch from a duplicated `#[cfg(debug_assertions)]` to a single
  expression.
- F20: while porting, simplify `if a { false } else { true }` → `!a` and
  `if dot { true } else { false }` → `dot`; drop the
  `#[allow(clippy::if_same_then_else, clippy::nonminimal_bool)]` attribute.
- F29: delete the 105-line commented-out mupdf-port block at
  `oxidize.rs:46-150`.
- F22 (partial): add an in-tree `#[test]` that exercises the new
  `oxidize::extract_pages` against a synthetic PDF, so CI can verify the
  port without the SSH-only test-data submodule.

**Out of scope** (covered by separate plans):

- F3 (`Op::Insert/Delete/Move/Replace` execution in `source.rs:80-90`).
- F4 (`PageKind::MageSpell` parsing).
- F5 (`GiftKind::Moon` arm).
- F6 (`.unwrap()` on `parse_merits`).
- F9 (`META.unwrap()`).
- F10, F11, F12 (`meta-editor` UI bugs).
- F13 (`let str = …` shadows).
- F15 (walker/cache dedup).
- F17, F19 (lint-supppression placements).
- F21 (hash canonicalization in the test).
- F23–F28, F30, F31 (hygiene).
- F32 (stable-Rust blocker via `cofd-util` `generic_const_exprs`).

## 3. Architecture (after)

```
crates/cofd-miner/src/backend/
  mod.rs       — cfg-gated mod + pub use for both backends
  oxidize.rs   — full algorithm driven by TextFragment
  mupdf.rs     — unchanged
  (lopdf.rs)   — DELETED
```

Public surface unchanged: `cofd_miner::extract_pages` resolves to
`oxidize::extract_pages` on the default build and to `mupdf::extract_pages`
under `--features mupdf`. `PdfText = BTreeMap<usize, Vec<String>>` continues
to be the type `source::process_section` consumes, so no other crate in the
workspace needs to change.

## 4. Design

### 4.1. `oxidize::extract_pages` signature

```rust
pub fn extract_pages(path: impl AsRef<Path>) -> anyhow::Result<PdfText>
```

Identical to `mupdf::extract_pages` (`crates/cofd-miner/src/backend/mupdf.rs:15`).

### 4.2. Algorithm — port of `mupdf.rs:23-128`

The five state-machine variables from `mupdf.rs:29-79` carry over verbatim;
only the **input type** changes. The column threshold becomes `f64` because
`TextFragment.x` and `TextFragment.y` are `f64`
(`oxidize-pdf-3.1.2/src/text/extraction.rs:131-132`).

| mupdf source (`mupdf.rs:38-67`)                              | oxidize source (this plan)                                |
| ------------------------------------------------------------ | --------------------------------------------------------- |
| `Document::open(path_str)?`                                 | `PdfDocument::open(path.as_ref())?`                       |
| `for page in document.pages()?`                             | `for i in 0..doc.page_count()?`                           |
| `page.to_text_page(TextPageOptions::empty())?`              | `doc.extract_text_from_page_with_options(i, options)?`    |
| `for block in text_page.blocks() { for line in block.lines() { ... } }` | `for fragment in extracted_text.fragments { ... }`         |
| `line.bounds().x0`, `line.bounds().y0`                      | `fragment.x`, `fragment.y`                                |
| `line.chars().filter_map(char).collect::<String>()`         | `fragment.text.clone()`                                   |
| 3-deep iteration (page → block → line)                      | flat iteration                                            |

Page-blank detection (`mupdf.rs:43-48`): identical `(y - last_y).floor().abs() > 100.0` rule. The sign of `y` is not assumed — only the absolute jump magnitude, so a top-down vs bottom-up axis does not break this rule.

Per-fragment skip (`mupdf.rs:50-56`): drop all-numeric fragments and the
"blank" tail of each page, same as mupdf.

Column classification (`mupdf.rs:58-64`): the threshold becomes
`const THRESHOLD: f64 = 240.0;`; the same `l_indent`/`r_indent` min-tracker
runs.

Indent-tab decision (`mupdf.rs:94-103`): same formula, with F20
simplification applied in the same edit:

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

(The two outer `if-then-else` arms in the original are each already
booleans; collapsing them is a drive-by correctness/cleanup pass that the
audit called out as F20. The De Morgan's-law equivalent `!last_has_dot
|| dot` is used instead of `!(last_has_dot && !dot)` to avoid
`clippy::nonminimal_bool` under the crate's `#![warn(clippy::pedantic)]`
plus `-D warnings` gate. The two forms are algorithmically identical.)

`INDENT_DEBUG` branch (`mupdf.rs:120-127`): replaced with a single
`if std::env::var("INDENT_DEBUG").is_ok() { format!("{indent}\t{line}", …) }`
expression, no `#[cfg(debug_assertions)]` duplication (F18).

### 4.3. `ExtractionOptions`

```rust
use oxidize_pdf::text::ExtractionOptions;

let options = ExtractionOptions {
    preserve_layout: true,         // correctness-critical: without this, .fragments is empty
    sort_by_position: true,        // correctness-critical: stream-order scrambles the line state machine
    reconstruct_paragraphs: true,  // correctness-critical: without this, .fragments is per-Tj not per-line
    ..Default::default()            // merge_hyphenated: true, include_artifacts: false, etc.
};
```

All three flags are correctness-critical:

- `preserve_layout: true` populates `ExtractedText.fragments` with the
  per-`Tj` text fragments with their `(x, y)` positions. With the default
  `preserve_layout: false`, `extract_text_from_page_with_options` returns
  `ExtractedText { text: <string>, fragments: vec![] }` (per
  `oxidize-pdf-3.1.2/src/text/extraction.rs:105`: *"Text fragments with
  position information (if preserve_layout is true)"*). The algorithm
  iterates over `fragments`; an empty Vec produces an empty page.
- `sort_by_position: true` ensures fragments come out in `(x, y)` order
  instead of PDF content-stream order, so the line-reconstruction state
  machine sees adjacent baselines grouped together.
- `reconstruct_paragraphs: true` groups per-`Tj` fragments on the same
  baseline into single-line fragments, replicating mupdf's
  `TextPage.lines()` semantics. Without this, a visual line like
  "Matt Miller," is emitted as 11 single-character fragments and the
  per-line indent-tab state machine sees a fragmented input. The flag
  is documented in `oxidize-pdf-3.1.2/src/text/extraction.rs:52-63`:
  *"Reconstruct visual lines and paragraphs from the raw text fragments
  produced by PDF text-show operators. When `true`, the extractor groups
  fragments by baseline into single-line fragments..."* This option is
  forced on for `PdfDocument::partition*` entry points; we must opt in
  explicitly for `extract_text_from_page_with_options`.

The rest use defaults from
`oxidize-pdf-3.1.2/src/text/extraction.rs:81-98`.

### 4.4. Error handling

mupdf's `Document::open(...).ok_or(anyhow!("Path is not valid utf-8 string"))?`
is replaced — `oxidize_pdf::parser::PdfDocument::open` takes
`impl AsRef<Path>` directly, so the utf-8 cast goes away. Every
`PdfDocument::open` / `extract_text_from_page_with_options` / `page_count`
call wraps the error with `anyhow::Context::with_context(|| format!("opening {}", path.display()))` so failures are debuggable (drive-by on F9's general critique).

### 4.5. `backend/mod.rs` (revised)

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

- Removes the `// #[cfg(feature = "lopdf")]` commented block.
- Removes the `lopdf` cfg gate entirely.
- Both backends compile independently; `pub use` last-wins per cargo feature unification, so picking one at a time on the command line (e.g. `cargo run -p cofd-miner --no-default-features --features mupdf`) is the supported switch.

### 4.6. `Cargo.toml` (revised)

```toml
[features]
default = ["oxidize", "embed_meta"]
embed_meta = ["dep:serde_json", "dep:rmp-serde", "dep:cofd-meta"]

oxidize = ["dep:oxidize-pdf"]
mupdf = ["dep:mupdf"]

[dependencies]
...
oxidize-pdf = { version = "3.1", optional = true }
mupdf = { version = "0.4", default-features = false, optional = true }
...
```

- `oxidize-pdf` floor bumped from `"3.0"` to `"3.1"`. The lock resolves to
  3.1.2; the audit-suggested floor of 3.0 omits the
  `extract_text_from_page_with_options` method's exact signature (per
  `oxidize-pdf-3.1.2/src/parser/document.rs:1170`), so 3.1 is the safe lower
  bound.
- `mupdf` re-declared as a real optional dep with `default-features = false` (no `sys-lib`). The default feature set pulls in `font-kit`/`yeslogic-fontconfig-sys` which require `fontconfig` (not in the devenv) and `tesseract` (not needed for the PDF read path), and `sys-lib` requires a system mupdf with an ABI mupdf 0.4's bindings were generated against — nixpkgs ships `mupdf 1.27` which has different ABI. Disabling default features keeps only PDF reading via the vendored mupdf source that `mupdf-sys` ships and that the bindings are guaranteed to match.
- The `lopdf` commented block is removed.

## 5. Testing

### 5.1. Algorithm coverage

Algorithm coverage is verified by the post-merge smoke test (Task 5). The original plan included a synthetic-PDF in-tree test; the pre-flight review (see `progress.md` F-PRE-1) determined that the hand-rolled PDF's xref byte offsets were wrong, and the user elected to drop the in-tree test rather than compute correct offsets. Smoke test against `~/Documents/RPG/CofD/Books/` is the verification surface.

### 5.2. Integration test (unchanged)

`crates/cofd-miner/tests/pdf_extract.rs` is left `#[ignore]`'d. It depends
on the SSH-only test-data submodule and cannot run in CI; the post-merge
smoke test above is the verification surface.

### 5.3. Post-merge smoke test

On a host with the private submodule and the local `pdf/` directory:
`cargo run -p cofd-miner --example pdf-extract` followed by
`jq '.merits | length, .gifts | length' res/tests/data/*.json` to confirm
non-zero counts (closes the F22 "info-only fixtures" regression).

## 6. Verification (pre-merge gates)

1. `cargo check --workspace --all-targets --features mupdf,oxidize` — both
   backends compile.
2. `cargo test -p cofd-miner --lib` — confirms the crate's library test target builds; algorithm coverage is provided by the smoke test below.
3. `cargo clippy --workspace --all-targets --features mupdf,oxidize -- -D warnings` — no new warnings (the F20 simplification removes a
   `clippy::if_same_then_else`; the new code must not introduce others).
4. `cargo run -p cofd-miner --example pdf-extract` (submodule-initialized host only) — produces non-info-only `Book` JSON for at least one of
   the seven fixtures.

## 7. Risks

- **Axis orientation.** `TextFragment.y` is documented as "page coordinates"; whether that is top-down or bottom-up depends on the PDF's MediaBox orientation. The state machine only consumes the absolute jump `(y - last_y).floor().abs() > 100.0`, so axis direction does not affect blank detection. The post-merge smoke test on a real fixture surfaces any remaining orientation problem.
- **Fragment-per-`Tj` performance.** `oxidize-pdf 3.1` emits one
  `TextFragment` per `Tj` operator; the 400-page Vampire PDF will produce
  more fragments than mupdf's per-line grouping. The per-fragment state
  machine is O(1) and runs in a `Vec::iter()`, so asymptotic complexity is
  unchanged. v1 concern; profile if it bites.
- **oxidize-pdf minor-version drift.** Floor at 3.1; lock resolves to
  3.1.2. If a 3.2 changes `TextFragment` field order or
  `extract_text_from_page_with_options` arity, the `cargo check` gate
  catches it.

## 8. File-level change summary

| Path                                                           | Change                                                                                          |
| -------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `crates/cofd-miner/Cargo.toml`                                 | Bump `oxidize-pdf` floor to `3.1`; re-declare `mupdf` optional dep with `default-features = false` (no `sys-lib` due to ABI mismatch; default fontconfig-dependent features are unavailable in the devenv); drop commented `mupdf`/`lopdf` blocks. |
| `crates/cofd-miner/src/backend/mod.rs`                         | Drop `lopdf` cfg; keep `mupdf` and `oxidize` cfg; drop commented `lopdf` block.                 |
| `crates/cofd-miner/src/backend/oxidize.rs`                     | Full algorithm port (≈100 lines, all live). Drop 105-line commented block. Drop `println!`. No in-tree test; algorithm coverage is via the post-merge smoke test. Apply F20 + F18 cleanups. |
| `crates/cofd-miner/src/backend/mupdf.rs`                       | Unchanged.                                                                                      |
| `crates/cofd-miner/src/backend/lopdf.rs`                       | Deleted.                                                                                        |

No other files in the workspace change. `cofd-miner` re-exports
(`extract_pages`, `extract_text`, `process_section` from
`crates/cofd-miner/src/lib.rs:25`) keep the same signatures.
