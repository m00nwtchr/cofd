# Architecture Decision Record — cofd monorepo

> Persisted to `codebase-memory-mcp` on 2026-07-24 against project `cofd`.
> This file is the source of truth; the MCP store is a cross-session cache.

## PURPOSE

The cofd monorepo houses a Rust toolchain for working with Chronicles
of Darkness (CofD) RPG material:

- Parse hand-curated SourceMeta JSON files into typed CofD book
  representations (`cofd-schema::Book`).
- Extract structured content from user-owned PDFs via the
  `cofd-miner` extraction pipeline (PDF backend: `oxidize-pdf`).
- Pull structured content from the public `codexofdarkness.com` wiki
  via the `codex-scraper`.
- Drive a Dioxus-based reader/editor UI (`cofdtools`) for browsing
  merits, gifts, spells, etc.

The monorepo also embeds an `attribute / modifier / supplier`
library (`systema`) consumed as a path-dependency from outside the
workspace. `systema` is intentionally generic (no RPG-flavored types)
so it can be reused by other projects.

## STACK

- Rust 2024 edition across all crates.
- Stable toolchain (`rust-toolchain.toml`).
- `devenv` shell (declarative Nix) at the workspace root provides
  native deps for PDF parsing and a unified dev environment.
- Workspace: single root `Cargo.toml` listing 10 leaf crates; no
  nested workspaces.
- `systema` consumed via `[workspace.dependencies] systema = { path =
  "../systema", features = ["serde"] }`.
- `cofd-miner-test-data` is a private git submodule at
  `crates/cofd-miner/res/tests/data` (SSH-only, `update = none`);
  it contains copyrighted SourceMeta inputs that must never be
  vendored, mirrored publicly, or pushed elsewhere.
- Serialization: `serde` + `facet` (in-progress migration to
  `facet` on the `facet` branch of cofd-rs).
- UI: Dioxus 0.7 (desktop + web), eframe/egui 0.25 (meta-editor),
  Tailwind CSS.
- Indexing: `codebase-memory-mcp` knowledge graph over the
  workspace root (1380 nodes / 3786 edges at the time of writing).

## ARCHITECTURE

Layered, dependency-rings-out-from-schema architecture:

1. **Core layer** — `cofd-schema`. Plain data types (`Book`,
   `Item<T>`, `Gift<T>`, `Merit`, `Spell`, `BookReference`,
   `DotRange`, `Prerequisites`, `DicePool`, `SuggestedModifiers`,
   template per splat). 67 inbound calls, 2 outbound — every
   other crate imports from here. Zero CoFD-specific runtime logic;
   pure data shapes + traits.
2. **Foundation layer** — `systema` (external). Generic
   attribute/modifier engine. `AttributeInstance<V, K, O>`,
   `AttributeMap`, `AttributeSupplier`/`AttributeSupplierBuilder`,
   `AttributeModifier`. `Value::Attribute(other_key)` lets
   modifiers depend on other attribute values; `mark_dependents_dirty`
   does iterative cache invalidation along the dependency graph
   (FINDINGS §2 fix).
3. **Engine layer** — `cofd` (the lib at `crates/cofd/`,
   package name `cofd`). Character + splat model. Brings in
   `cofd-schema` for types, `cofd-util` + `cofd-macros` for traits
   + proc-macros, `systema` for the attribute engine. Currently
   has 40 build errors on the `facet` branch; the `cofd-miner`
   surface is the only stable part.
4. **Build macros** — `cofd-derive` (SplatEnum, VariantName,
   AllVariants, NameKey) + `cofd-macros` (gifts!, merits!,
   expand_enum). `cofd-derive` is currently broken (AllVariants
   derive impl returns `&[T;N]` from `fn all() -> Vec<Self>`) —
   Tier-2 fix.
5. **Small utilities** — `cofd-util` (trait re-exports) and
   `cofd-meta` (SourceMeta types + serde_json helpers for the
   meta files).
6. **Extraction layer** — `cofd-miner` (PDF → Book pipeline).
   Backends in `crates/cofd-miner/src/backend/`; parsers in
   `crates/cofd-miner/src/parse/`; source extraction in
   `crates/cofd-miner/src/source.rs`. The `embed_meta` build
   feature compiles the per-book meta JSON files at
   `crates/cofd-miner/res/tests/data` (submodule) into a single
   `meta.bin`; `parse_book()` looks up a hash → meta mapping at
   runtime.
7. **Editor / scraper layer** — `meta-editor` (egui GUI for
   hand-curating meta files; the operator's authoring tool) and
   `codex-scraper` (reqwest + scraper; pulls structured content
   from the public wiki).
8. **Reader layer** — `cofdtools` (Dioxus 0.7 reader).
   Persists the user's library to
   `~/.local/share/CofD.tools/books.json` via `directories`.
   Surfaces "Files needing metadata" when a PDF upload has no
   matching meta JSON (Tier-1 fix B).

Cross-crate edges (from MCP boundary analysis):

- `cofd-miner → cofd-schema` (25 calls)
- `cofd → cofd-schema` (24 calls)
- `codex-scraper → cofd-schema` (10 calls)
- `meta-editor → cofd-miner` (6 calls)
- `cofdtools → cofd-schema` (3 calls)
- `cofd-derive → cofd` (3 calls)

## PATTERNS

- **Workspace inheritance**: every crate uses
  `version.workspace = true` / `edition.workspace = true` / dep
  `xyz.workspace = true`. Centralised in the root `[workspace]`
  block — no per-crate drift.
- **Crate dir == package name**: `crates/cofd-schema/` houses the
  `cofd-schema` package, `crates/cofd/` houses the `cofd` package,
  etc. Single source of truth for naming.
- **Workspace deps for path-override of inter-crate refs**:
  `cofd-schema = { path = "crates/cofd-schema" }`, etc. Inner
  crates use `cofd-schema.workspace = true`.
- **schema = single source of truth for data shapes**: every
  consumer of CofD content goes through `cofd-schema::Book`; no
  crate invents its own representation of a merit, gift, etc.
- **SourceMeta is an external input format**: hand-curated JSON
  per book lives outside the workspace (in a private submodule)
  and is consumed by `cofd-miner` to drive parsing. This keeps
  copyrighted content out of the open repo.
- **Build-time embedding**: `cofd-miner/build.rs` packs all
  SourceMeta JSON into a single `meta.bin` when the
  `embed_meta` feature is on. Hash-based lookup at runtime means
  the binary only needs one data file plus the user's PDFs.
- **Honcho / MCP for cross-session state**: tier-1 outcome notes,
  private-submodule restriction, and per-session insights are
  stored in Honcho so future sessions can recover context.
- **Iterative attribute invalidation** (post-FINDINGS §2 fix):
  `mark_dependents_dirty` uses a `HashSet<A>` worklist + `Vec<A>`
  queue instead of unguarded recursion. Cycle-safe.

## TRADEOFFS

- **Single Cargo workspace vs per-crate repos**: chose single
  workspace. Pros: one lockfile, one CI, one devenv, atomic
  cross-crate refactors, the `systema` and `facet` work-in-progress
  no longer requires GitHub tag pushes to land. Cons: one git
  repo, one CI per project (until we re-introduce per-crate CI
  triggers); the private submodule is wired in only locally.
- **`facet` vs `serde`**: `facet` is in-progress migration target
  on the cofd-rs `facet` branch. Mid-refactor state produces
  40 build errors in `cofd` lib; `cofd-schema` and the
  extraction pipeline still work. We pinned consumers to
  `cofd-schema-v0.1.0` / `cofd-miner-v0.1.0` tags on the last
  pre-`facet` commits so downstream isn't broken while the
  refactor continues.
- **`oxidize-pdf` vs `mupdf` vs `lopdf`**: switched to
  `oxidize-pdf 3.0` (the active default); `mupdf`/`lopdf` paths
  are commented out in the Cargo.toml. The switch required
  nightly toolchain.
- **Dioxus 0.7 + `cofdtools` runtime**: `books.json` is the only
  persistent state. On first run, falls back to empty defaults.
  NoSQL-style; no migrations.
- **Submodule + SSH only**: the test-data submodule is the
  *only* part of the workspace that is not buildable from a
  fresh clone. The constraint is intentional (copyrighted
  material) and documented in `ECOSYSTEM.md`.
- **`update = none` on the submodule**: pinning by hash is
  deliberate. Don't run `git submodule update --remote`.

## PHILOSOPHY

- **Data first, behaviour second**: the schema layer is the
  authority on what CofD content *is*; every other layer either
  consumes that representation or fails.
- **Self-hosted data**: never assume the wiki / a CDN / another
  repo will be reachable. PDFs the user owns are the source of
  truth; the codex wiki is a fallback.
- **Tiered correctness**: Tier-1 fixes ship small, isolated,
  testable changes that unblock work across the whole ecosystem
  (path overrides, tag pinning, monorepo restructure). Tier-2 and
  Tier-3 (full `facet` migration, systema value-cache hardening,
  cofd-miner backend stabilisation) are documented but deferred
  until Tier-1 unblocks contributors.
- **Tooling augments, doesn't replace**: `meta-editor` exists to
  hand-curate `SourceMeta`. The pipeline is dumb about content
  semantics; humans tag pages, types enforce structure.
- **Honest documentation**: `FINDINGS.md` files, `TIER-1-RESULTS.md`,
  and `ECOSYSTEM.md` document what is broken, what is fixed,
  and what is unknown. Honcho captures cross-session
  constraints (private submodule, etc.) so they survive
  agent restarts.