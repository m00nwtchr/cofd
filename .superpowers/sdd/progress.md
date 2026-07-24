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

