# CofD Ecosystem

This directory is the planned monorepo root for the Chronicles-of-Darkness toolchain.
Three sibling repositories are being consolidated here:

- `cofd-rs/` — domain model + character engine (the `cofd` repo on GitHub)
- `cofd-miner/` — PDF extraction pipeline (the `cofd-miner` repo on GitHub)
- `cofdtools/` — Dioxus reader application

`systema/` lives at `../systema/` (a sibling repo) and is consumed via Cargo
`[patch]` from inside this workspace.

## Release / pinning policy

- `cofd-rs/master` carries the long-running `facet` refactor. It is **not**
  pinned by downstream consumers.
- `cofd-miner` and `cofdtools` pin to tagged releases of `cofd-schema` and
  `cofd-miner` (see below).
- Tags are immutable; once published, downstream may rely on them.
- A breaking change to `cofd-schema` requires a new tag (`cofd-schema-v0.2.0`,
  `cofd-schema-v0.3.0`, …) and updates to consumer `Cargo.toml` files.

### Current pinned tags

| Consumer | `cofd-schema` | `cofd-miner` |
|---|---|---|
| `cofd-miner/Cargo.toml` | `cofd-schema-v0.1.0` | (workspace-internal) |
| `cofdtools/Cargo.toml` | `cofd-schema-v0.1.0` | `cofd-miner-v0.1.0` |

Both `cofd-schema-v0.1.0` and `cofd-miner-v0.1.0` were tagged on
2026-07-24 from the last pre-`facet` / pre-`oxidize-pdf` commits on the
respective `master` branches.

## Layout (after Tier-1 fixes complete)

```
cofd/                              ← this directory (workspace root)
├── Cargo.toml                     ← root workspace manifest
├── Cargo.lock                     ← unified lockfile
├── ECOSYSTEM.md                   ← this file
├── devenv.nix                     ← shared dev environment
├── .gitignore
├── cofd-rs/                       ← domain model (was `cofd/`)
│   ├── Cargo.toml                 ← inner workspace manifest
│   └── schema/  lib/  derive/  macros/  util/
├── cofd-miner/                    ← PDF extraction
│   ├── Cargo.toml                 ← inner workspace manifest
│   ├── crates/
│   │   ├── miner/                 ← core extraction
│   │   ├── meta/                  ← SourceMeta types
│   │   ├── meta-editor/           ← egui GUI for meta files
│   │   └── codex-scraper/         ← codexofdarkness.com scraper
│   └── meta/                      ← per-book SourceMeta JSON files
└── cofdtools/                     ← Dioxus reader app
    ├── Cargo.toml                 ← single-crate member of root workspace
    └── src/

../systema/                        ← standalone, consumed via [patch]
```

## Toolchain

- Rust: `nightly` (because `oxidize-pdf` and some `eframe` deps require it).
- Build system: `devenv` at the workspace root.
- Lint: `#![warn(clippy::pedantic)]` enabled in `cofd-rs/` and `cofd-miner/`.

## Tier-1 fix summary (this iteration)

| Fix | Description | Status |
|---|---|---|
| D.1 | Tag `cofd-schema-v0.1.0` on `cofd-rs/master` | done (local tag) |
| D.2 | Tag `cofd-miner-v0.1.0` on `cofd-miner/master` | done (local tag) |
| D.3 | Pin `cofd-miner` and `cofdtools` to those tags | done (working tree) |
| C.1 | Iterative `mark_dependents_dirty` in `systema/` | pending |
| B.1 | Graceful `NoSuchMeta` handling in `cofdtools/` | pending |
| A.1–A.6 | Monorepo restructuring | pending |

**Tag push is the operator's responsibility.** Before any CI / published
release, run from each repo:

```bash
git push origin <tag-name>
```

The current tags exist only in the local clones.