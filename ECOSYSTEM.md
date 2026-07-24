# CofD Ecosystem

This directory is the monorepo for the Chronicles-of-Darkness toolchain.
All Rust crates live under `crates/*`; systema lives at `../systema/` and
is consumed via a Cargo `[patch]` from inside this workspace.

## Layout

```
cofd/                                  ← workspace root
├── Cargo.toml                         ← single root manifest, 10 members
├── Cargo.lock                         ← unified lockfile
├── ECOSYSTEM.md                       ← this file
├── TIER-1-RESULTS.md                  ← outcome of the Tier-1 fix pass
├── devenv.nix                         ← shared dev environment
├── .gitignore
├── .codebase-memory/                  ← MCP graph cache (gitignored)
├── meta/                              ← SourceMeta JSON inputs for embed_meta
│   └── *.json
└── crates/
    ├── cofd-schema/                   ← book / item / template types
    ├── cofd/                          ← cofd-rs/lib: character engine
    ├── cofd-derive/                   ← proc-macro crate (SplatEnum, VariantName)
    ├── cofd-macros/                   ← proc-macro crate (gifts!, merits!)
    ├── cofd-util/                     ← small utility traits
    ├── cofd-miner/                    ← PDF extraction pipeline
    ├── cofd-meta/                     ← SourceMeta types + per-book JSON
    ├── meta-editor/                   ← egui GUI for hand-curating meta
    ├── codex-scraper/                 ← codexofdarkness.com wiki scraper
    └── cofdtools/                     ← Dioxus reader application

../systema/                            ← standalone repo, consumed via path
```

## Crate layout convention

- Directory name matches the published package name when possible.
- `cofd`, `cofd-schema`, `cofd-util`, `cofd-derive`, `cofd-macros`,
  `cofd-miner`, `cofd-meta`, `meta-editor`, `codex-scraper`, `cofdtools`.
- Every crate's `Cargo.toml` is self-contained — no nested workspaces.

## Release / pinning policy

- `cofd-rs/master` carries the long-running `facet` refactor. It is
  **not** pinned by downstream consumers.
- The two tags that exist today
  (`cofd-schema-v0.1.0`, `cofd-miner-v0.1.0`) were created during the
  Tier-1 fix pass on 2026-07-24 to document the last pre-`facet`
  commits. They live on local-only git remotes (in `/tmp/cofd-history/`)
  since the upstream `m00nwtchr/cofd{,-miner,-tools}` repos are
  being consolidated into this monorepo.
- A breaking change to `cofd-schema` requires a new tag
  (`cofd-schema-v0.2.0`, …) and updates to consumer `Cargo.toml` files.

## Cross-crate dependency edges (within this monorepo)

```
cofd-schema ← cofd-util, cofd, cofd-macros, cofd-miner, cofd-meta,
              meta-editor, codex-scraper, cofdtools
cofd-util   ← cofd-schema, cofd
cofd-derive ← cofd-util
cofd-macros ← cofd-schema
cofd        ← cofd-schema, cofd-util, cofd-macros, systema
cofd-miner  ← cofd-schema, cofd-meta
cofd-meta   ← cofd-schema
meta-editor ← cofd-schema, cofd-meta, cofd-miner
codex-scraper ← cofd-schema
cofdtools   ← cofd-schema, cofd-miner

systema     (outside the monorepo; consumed via path = "../systema")
```

## Toolchain

- Rust: `stable` (per `devenv.nix`).
- Build system: `devenv` at the workspace root.
- Lint: `#![warn(clippy::pedantic)]` enabled in `cofd-schema`,
  `cofd-derive`, `cofd-macros`. `cofd` (lib) is excluded because of
  the in-progress `facet` refactor.