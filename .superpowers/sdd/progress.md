# Progress ledger — oxidize-pdf transition

Track per-task state. The ledger is the recovery map; trust it and `git log`
over your own context after compaction.

## Tasks

- Task 1: Port indent-reconstruction algorithm into oxidize.rs — PENDING
- Task 2: Update backend/mod.rs (drop lopdf) — PENDING
- Task 3: Delete lopdf.rs — PENDING
- Task 4: Update Cargo.toml (bump oxidize-pdf, re-declare mupdf) — PENDING
- Task 5: Workspace check + post-merge smoke test — PENDING

## Pre-flight findings (RESOLVED)

### F-PRE-1: Synthetic PDF in original Task 1 had wrong xref offsets
**Resolution:** Dropped the synthetic-PDF test entirely (user choice).
Algorithm coverage now comes from the post-merge smoke test (Task 5).
Audit F22 (partial) is therefore not closed by this plan.

### F-PRE-2: Task 1 step 1 "lines 15–152" range
**Resolution:** Non-blocking. Task 1 (revised) says to replace the entire
file body. No plan change needed.

### F-PRE-3: mupdf sys-lib doesn't work with nixpkgs mupdf 1.27
**Resolution:** Changed mupdf dep declaration to use default features
(vendored MuPDF build via mupdf-sys). First compile is 5–10 min but
the system mupdf isn't required. This sidesteps the nixpkgs ABI
mismatch with mupdf 0.4's FFI bindings.

### F-PRE-4: Smoke test corpus path
**Resolution:** Use `~/Documents/RPG/CofD/Books/` (user-confirmed). No
change needed.

Task 1: complete (commits 8e1e64a, 61c0efb, F20 lint resolved, 12 pre-existing cofd-miner clippy warnings out of scope)
Task 2: complete (commit aca410f, lopdf cfg dropped, both backends cfg-gated, cargo check exit 0)
Task 3: complete (commit 164ac89, lopdf.rs deleted, cargo check exit 0)
Task 4: complete (commit 94bbb18, oxidize-pdf 3.1 floor, mupdf default-features=false; mupdf build 80s, default build 0.14s; all single-backend feature combos work; --features mupdf,oxidize correctly errors per plan §4.5)
Task 5: DONE_WITH_CONCERNS — verification surfaced a substantive bug. The `ExtractionOptions` in `crates/cofd-miner/src/backend/oxidize.rs:39` is missing `preserve_layout: true`, so `extract_text_from_page_with_options` returns empty `fragments` for every page; the indent-reconstruction loop iterates over `extracted.fragments` (empty), so every page produces an empty line list and `PdfExtract::parse` produces empty `Book`. Smoke test ran without panic (EXIT=0 in ~30s) but regenerated all seven fixtures as info-only stubs. Submodule restored to HEAD. The spec §4.3 / Global Constraint line 26 are wrong: `preserve_layout: true` is also correctness-critical alongside `sort_by_position`. One-line fix needed; out of scope for verification-only Task 5. Full report: `task-5-report.md`.
Task 5: complete (commits 6c875de algorithm fixes, 37d7cd7 doc updates, 2b6a8b8 fixtures regenerated; 6/7 books now have non-info-only content vs 0/7 before; F6 parse_merits panic blocks Hurt Locker/CoD/Demon/Signs of Sorcery — separate P0)

## F6 fix and final parity status

F6 (parse_merits .unwrap() and find .unwrap()) fixed. Hurt Locker recovered 0 -> 120 merits.

Final oxidize vs mupdf (both with F6 fix):
- total merits: oxidize=320, mupdf=485 (oxidize gets 66% of mupdf's count)
- Vampire: oxidize=8, mupdf=95 (extraction layer text-format regression)
- Werewolf: oxidize=9+4 gifts, mupdf=69+18 gifts (same root cause)

F6 was the blocker for Hurt Locker only. The Vampire/Werewolf gap is from text-format
differences in the oxidize extraction layer (per-Tj granularity, missing spaces, line break
drift) that prevent the meta-JSON's regex from matching merit headers. This is the same
issue identified in the post-merge parity check; the F6 fix doesn't address it.
