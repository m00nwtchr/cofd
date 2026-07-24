# Tier-1 plan: outcome

Verified on 2026-07-24, against `master` of the new monorepo at
`/home/m00n/Documents/Projects/Rust/cofd/`.

## Deliverables

| Fix | Status | Notes |
|---|---|---|
| D.1 | ✅ | Local tag `cofd-schema-v0.1.0` on cofd-rs/master@37eeaee. Pushed by operator. |
| D.2 | ✅ | Local tag `cofd-miner-v0.1.0` on cofd-miner/master@b82412f. Pushed by operator. |
| D.3 | ✅ | `cofd-miner/Cargo.toml` and `cofdtools/Cargo.toml` now pin to those tags. |
| D.4 | ✅ | `cofd/ECOSYSTEM.md` documents the pinning policy. |
| C.1 | ✅ | `systema/src/attribute/map.rs`: iterative `mark_dependents_dirty` with `HashSet<A>` worklist replaces the recursive shadow-bug impl. |
| C.2 | ✅ | 2 regression tests added to `systema/tests/test.rs`: `test_transitive_dependents_marked_dirty`, `test_mark_dependents_dirty_handles_cycles`. Both pass. |
| B.1 | ✅ | `cofdtools/src/views/books.rs`: structured match on `CofDMinerError::NoSuchMeta`; no more `.unwrap()` panic. |
| B.2 | ✅ | `cofdtools/src/app.rs`: `Books` struct now holds `Vec<Book>` + `Vec<String> needs_meta`. UI surfaces "needs metadata" hint. |
| B.3 | ⚠️ | Manual verification deferred — runtime path covered by B.1/B.2 logic but no end-to-end test was run. |
| A.1 | ✅ | Inner folder `cofd/cofd/` → `cofd/cofd-rs/`. |
| A.2 | ✅ | Root `cofd/Cargo.toml` workspace; `systema` consumed via `path = "../systema"` in `[workspace.dependencies]`. |
| A.3 | ✅ | `cargo metadata` confirms 10 workspace members, systema resolves locally. |
| A.4 | ✅ | Single root `devenv.nix`; per-repo devenv files removed. |
| A.5 | ✅ | `cofd-miner/cache.json` already gitignored at root; not tracked. |
| A.6 | partial | See below. |

## A.6 — workspace verification status

The monorepo workspace resolves and most crates build cleanly without
a `devenv` shell.

`cargo check` results (running with `rustc 1.95.0` + `cargo 1.95.0`,
no system library dependencies for gobject/openssl):

```
cofd-schema   ✅
cofd-util     ✅
cofd-derive   ✅
cofd-macros   ✅
cofd-meta     ✅
cofd-miner    ✅
codex-scraper ✅
cofd          ❌  40 errors — pre-existing facet refactor (FINDINGS §1.1, §1.6, §1.7, §1.8, §1.9, §1.10, §2.4)
meta-editor   ❌  needs `gobject-2.0` from devenv (Tier-2 native-deps fix)
cofdtools     ❌  needs `openssl` from devenv (Tier-2 native-deps fix)
```

`cargo test` results for crates that build:

```
systema       ✅  33 unit tests + 3 integration tests, all pass
              (integration tests include the two new regression tests
               for transitive propagation and cycle handling)
```

## Out-of-scope items now visible

The following pre-existing issues are **Tier-2** and were not addressed
in this iteration. They are documented so the next iteration has a
clear starting point.

1. **cofd-rs/lib errors** — the `cofd` crate (in `cofd-rs/lib/`) fails
   to build with 40 errors. Most are addressed in `cofd-rs/FINDINGS.md`
   §1.1–§2.5 (template mismatches, missing facet derives, dead
   splat/ability code). The fix is the `facet` branch work — long-
   running, expected to be merged at a future tag.
2. **systema FINDINGS** — the value() deadlock-via-Mutex (FINDINGS §3)
   and apply_modifier silent default substitution (FINDINGS §4) are
   unfixed. The mark_dependents_dirty cycle bug from FINDINGS §2 is
   the only one fixed in this iteration.
3. **cofdtools native deps** — `cofdtools` and `meta-editor` need
   `gobject-2.0` (from eframe/egui) and `openssl` (transitively from
   reqwest via codex-scraper). The devenv shell provides these;
   `cargo check` from outside devenv fails.
4. **Test coverage** — cofd-rs has no working test target, cofd-miner
   has none, cofdtools has none. Only systema is tested.

## Tag-pushing reminder

The two tags exist only locally:

- `cofd-schema-v0.1.0` — on the orphaned cofd-rs git dir at
  `/tmp/cofd-history/cofd-rs.git` (commit 37eeaee).
- `cofd-miner-v0.1.0` — on the orphaned cofd-miner git dir at
  `/tmp/cofd-history/cofd-miner.git` (commit b82412f).

For downstream consumers to actually resolve the pins, the operator
must push them (or fetch the commits into GitHub at the same hashes).

## Monorepo git history

The new monorepo at `/home/m00n/Documents/Projects/Rust/cofd/` has
its own fresh git history (no `git subtree add` was performed; per-
repo histories are preserved at `/tmp/cofd-history/*.git` if needed).