# Audit: `cofd-miner` — 2026-07-24

Static + graph-assisted audit of the `cofd-miner` workspace at
`/home/m00n/Documents/Projects/Rust/cofd/cofd-miner`, branch `master`,
HEAD `b82412f6`. No code changes were made; this document is the only
artifact produced.

Scope:

- 4 workspace crates: `cofd-miner`, `cofd-meta`, `meta-editor`,
  `codex-scraper`.
- External git dep: `cofd-schema` (commit `89b07a2`, from
  `github.com/m00nwtchr/cofd`).
- Sources cross-checked both with direct file reads and the
  `codebase-memory-mcp` knowledge graph (project `cofd-miner`, plus
  `cofd-rs` for `cofd-schema` types).

Conventions used in this document:

- **P0 / P1 / P2** — priority: extractor broken → robustness / quality →
  cleanup-only.
- **Verified** — directly checked in the knowledge graph for the
  project. **Read** — verified by reading the file.

---

## 1. Headline summary

The crate advertises itself as a PDF→structured-book extractor but the
**default build currently extracts nothing**: the `oxidize` backend is a
14-line stub that prints chunks to stdout and returns an empty
`BTreeMap`, and the only fully-implemented backend (`mupdf`) is
feature-flagged off and not even declared as an optional dependency in
`Cargo.toml`. Beyond this, several meta-JSON op variants and two
`PageKind` variants are parsed but never executed/dropped silently.
Documentation and examples assume the `mupdf` backend is wired up; the
code as it stands cannot fulfill the README's promise.

A second, smaller class of issues: external hygiene — submodule on SSH,
`pdf` symlink committed, `llm/` and `codex-scraper` directories that are
vestigial, dead `lopdf.rs` referencing an undefined `PageText` type.

---

## 2. Crate map

| Crate                       | Path                          | Role                                                                  | Build state                                          |
| --------------------------- | ----------------------------- | --------------------------------------------------------------------- | ---------------------------------------------------- |
| `cofd-miner`                | `crates/miner`                | PDF→`Book` extraction. Public lib + `pdf-extract`/`test` examples.    | Default features `oxidize + embed_meta`.             |
| `cofd-meta`                 | `crates/meta`                 | `SourceMeta` JSON schema (`SectionMeta`, `Op`, `PageKind`, range).    | Embedding tooling via `build.rs` of `cofd-miner`.    |
| `meta-editor`               | `crates/meta-editor`          | egui/eframe GUI for hand-authoring `meta/*.json`. `publish = false`.  | `eframe 0.25` (glow), no wgpu.                       |
| `codex-scraper`             | `crates/codex-scraper`        | Scrapes the codexofdarkness.com wiki. Vestigial, partially broken.   | `reqwest 0.12` (rustls-tls-webpki-roots).             |
| *external* `cofd-schema`    | `~/.cargo/git/checkouts/cofd` | `Book`, `Item`, gifts/merits/spells, prerequisites, dice, dot range. | Pinned commit `89b07a2cfb7d11b9a829f831c97baf63246aac8e`. |

Dependency count: **468 transitive packages** in `Cargo.lock`; ~22 Rust
files across the 4 crates.

Topologically:

```
meta-editor ──► cofd-miner ──► cofd-schema
       │           │
       └─► cofd-meta ┘  (meta used at compile time via include_bytes)
                                  ▲
codex-scraper ────────────────────┘
```

---

## 3. Critical-path findings (P0)

### F1 — Default `oxidize` PDF backend is a stub. *Verified.*

**File:** `crates/miner/src/backend/oxidize.rs:15-153` (**Read**)
**Graph:**
`cofd-miner.crates.miner.src.backend.oxidize.extract_pages` —
`complexity=1`, `cognitive=1`, `loop_count=1`, 139 source lines, only
22 of them active.

Active code (22 lines): opens the PDF via `oxidize_pdf::parser::PdfDocument::open`,
calls `doc.rag_chunks()?,` prints each chunk to stdout via
`println!("---\n{}\n---", chunk.text)`, returns an empty
`BTreeMap::new()`. The remaining 117 lines are commented-out indentation
reconstruction logic ported from `mupdf.rs` and never reactivated.

This is the **only** enabled backend (`Cargo.toml:11 default = ["oxidize",
"embed_meta"]`). Users running `cargo run --bin pdf-extract` get empty
pages and therefore empty `merits`/`gifts` output. The seven test
fixtures under `crates/miner/res/tests/data/*.json` are all 6–8 lines of
just `{"info": {...}}` — they are *exactly* what this stub backend
produces.

### F2 — `mupdf` backend is feature-flagged but undeclared. *Verified.*

**File:** `crates/miner/Cargo.toml:11-26` (**Read**)
**File:** `crates/miner/src/backend/mod.rs:6-17` (**Verified**)

`[features] default = ["oxidize", "embed_meta"]` — `mupdf` is not in
`default`. The commented-out lines at `Cargo.toml:15, 23-25` still imply
a `mupdf` feature gate, but `mupdf` is not declared under `[dependencies]`
(only `oxidize-pdf` is, and it's optional).

`backend/mod.rs:6, 14` does still `#[cfg(feature="mupdf")]` gate
`mupdf::extract_pages`, and `mod.rs:6-8` still includes the module. So
`--features mupdf` cannot compile cleanly even though the source tries
to honor the flag.

`mupdf.rs` has full indent-reconstruction logic (`complexity=18`,
`cognitive=54`, the only working backend) but is unreachable at compile
time.

### F3 — `Op::RegexReplace` is the only op variant executed. *Verified.*

**File:** `crates/miner/src/source.rs:80-90` (**Verified**)

```rust
for op in &section.ops {
    #[allow(clippy::single_match)]
    match op {
        Op::RegexReplace { regex, replace } => {
            extract = regex.replace_all(&extract, replace).into_owned();
        }
        _ => {}
    }
}
```

`Op::Insert`, `Op::Delete`, `Op::Move`, and `Op::Replace` (defined at
`crates/meta/src/lib.rs:17-38`) are dropped. The only meta file using a
non-`RegexReplace` op is `meta/Vampire the Requiem 2e.json:19-30`, with
two `Op::Insert { pos: 33016, char: '(' }` and `pos: 33023, char: ')'`.
Those ops' effects are silently lost.

### F4 — `PageKind::MageSpell` is dropped without dispatch. *Verified.*

**File:** `crates/miner/src/parse/mod.rs:27-56` (**Verified**)

The outer match on `section.kind` has arms for `Merit` and `Gift`, and a
catch-all `_ => {}`. `PageKind::MageSpell` falls into the catch-all.
The commented-out body at `parse/mod.rs:37-39` shows what was intended.

`meta/Mage the Awakening 2e.json:53-61` defines a section
`"name": "Death"`, `pages: {start:128, end:134}`, `kind: "MageSpell"`.
That section is silently ignored. The corresponding `Book::mage_spells`
field exists in the schema (`cofd-rs.schema.src.book.Book`) but
nothing here populates it.

### F5 — `GiftKind::Moon` is a no-op branch. *Verified.*

**File:** `crates/miner/src/parse/mod.rs:40-47` (**Verified**)

```rust
GiftKind::Moon => {
    // parse.moon_gifts.extend(todo!())
}
```

The branch body is empty. `meta/Werewolf the Forsaken 2e.json:64-73`
defines a `Moon Gifts` section — those gifts would be lost even if the
PDF backend were returning text.

### F6 — `.unwrap()` panics on the merit-parse hot path. *Verified.*

**File:** `crates/miner/src/parse/mod.rs:34-36` (**Verified**)

```rust
PageKind::Merit(_) => parse
    .merits
    .extend(parse_merits(&parse.info, &section).unwrap()),
```

`parse_merits` returns `Result<Vec<MeritItem>>` but is `.unwrap()`'d,
so any regex miss or unrecognized prerequisite string crashes the whole
extraction. By contrast, the parallel branch (`parse_gifts(...)?`) is
properly propagated.

### F7 — Meta JSON's `BookInfo.hash` mismatch with how it's parsed.

*Retracted after verification — see §6.*

Original worry: the on-disk JSON stores `"hash": "A4B653D08D7503AF"` as a
string, while `crates/miner/src/lib.rs:42-44 get_meta(hash: u64)` would
suggest `BookInfo.hash` is `u64` — so `String.eq(&u64)` would always be
`false`. Verified via the `cofd-rs` knowledge graph:

`cofd-rs.schema.src.book.BookInfo`:
```
pub hash: u64,
#[cfg_attr(feature = "serde", serde(with = "hex"))]
```

The serde layer (`#[serde(with = "hex")]`) round-trips the hex string to
and from `u64`, so both sides are `u64` and the comparison in `get_meta`
is correct. No bug here. **Retracted.**

---

## 4. Other functional / correctness findings (P1)

### F8 — `lopdf.rs` references undefined type `PageText`. *Verified.*

**File:** `crates/miner/src/backend/lopdf.rs:7-27` (**Verified**)

Return type is `anyhow::Result<PageText>`, but the only alias defined in
`crates/miner/src/backend/mod.rs:19` is `pub type PdfText =
BTreeMap<usize, Vec<String>>;`. There is no `PageText`.

Additional concerns:
- The final `.collect();` ends with bare `();` (line 26) — the function
  doesn't return a `Result<...>`, it returns `()`.
- Even setting type aside, `.collect()` of an iterator yielding
  `Result<(u32, Vec<String>)>` into `Result<PageText, _>` will fail to
  infer because `PageText` doesn't exist.

Enabling `--features lopdf` will not compile. The backend is also
not declared as an optional dependency in `Cargo.toml`.

### F9 — `from_slice(META_BYTES)` panics at process start. *Verified.*

**File:** `crates/miner/src/lib.rs:30-32` (**Read**)

```rust
#[cfg(feature = "embed_meta")]
static META: Lazy<Vec<SourceMeta>> =
    Lazy::new(|| rmp_serde::decode::from_slice(META_BYTES).unwrap());
```

If `OUT_DIR/meta.bin` is malformed or schema-incompatible, every binary
that calls `get_meta` panics at startup with no context.

### F10 — `meta-editor` Save ignores the `Result` from `serialize`. *Read.*

**File:** `crates/meta-editor/src/main.rs:265-271` (**Read**)

```rust
if ui.button("Save").clicked() {
    let mut ser = serde_json::Serializer::with_formatter(
        File::create(&self.meta_path).unwrap(),
        PrettyFormatter::with_indent(b"\t"),
    );
    self.meta.serialize(&mut ser);
}
```

Two failure modes:
1. `File::create(...).unwrap()` — panics if the file is not writable or
   the directory is read-only.
2. `self.meta.serialize(&mut ser)` returns `Result<()>` that is dropped.
   Any serialization error (e.g. invalid regex with `serde_regex`) is
   silently lost.

The user gets no feedback either way.

### F11 — `meta-editor` page inputs panic on non-numeric input. *Read.*

**File:** `crates/meta-editor/src/main.rs:195, 205` (**Read**)

```rust
section.pages =
    (self.pages_start.parse().unwrap())..=*section.pages.end();
```

If the user types "a" into the start-page field, the app panics. Should
be silently rejected or surfaced via `egui::Color32::RED`.

### F12 — `meta-editor` calls `extract_pages` on startup. *Read.*

**File:** `crates/meta-editor/src/main.rs:76` (**Read**)

```rust
let pages = cofd_miner::extract_pages(&path).unwrap();
```

Currently resolves to the `oxidize` stub, returning an empty
`BTreeMap::new()` — the GUI launches with no data to display. Will be
fine once F1 is fixed, but the `.unwrap()` pattern means F1 currently
saves us from immediate panic.

---

## 5. Code-quality & hygiene findings

### F13 — `str` shadows `std::str` in six places. *Read.*

Search `grep -rn "let str = "`. Hits:

- `crates/codex-scraper/examples/scrape.rs:37`
- `crates/codex-scraper/src/gifts.rs:64`
- `crates/miner/src/parse/mod.rs:178` (parameter), 183
- `crates/miner/src/parse/item.rs:190`
- `crates/miner/src/parse/mod.rs:178` (`fn normalize(str: &str)`)

Will trigger `clippy::shadow_unrelated` under `clippy::pedantic`, which
`crates/miner/src/lib.rs:1` enables. The compiler is fine with it
because `std::str` isn't imported, but readability suffers.

### F14 — `println!` left in production paths. *Read.*

- `crates/miner/src/backend/oxidize.rs:32` — prints every PDF chunk to
  stdout. This is the *default* code path; users running `pdf-extract`
  will see ~thousands of lines of PDF text scroll by.
- `crates/meta-editor/src/main.rs:242, 331` — `println!` used for user
  feedback in a GUI process (no visible output, goes to the spawned
  terminal).
- `crates/codex-scraper/src/merits.rs:40` — debug print before a `panic!()`.

### F15 — Duplicated "walk + hash cache" boilerplate. *Verified.*

The same `Cache` struct + `WalkDir` + `is_hidden`/`is_pdf` logic appears
three times:

- `crates/miner/examples/pdf-extract.rs:23-72`
- `crates/miner/tests/pdf_extract.rs:17-52`
- `crates/meta-editor/src/main.rs:46-76` (variant with regex scan of
  `meta/`)

Worth extracting into a small `cofd-miner::hashing_cache` helper.

### F16 — Dead comment block in `source.rs`. *Read.*

`crates/miner/src/source.rs:28-40` — 12 lines of commented-out refactor
(`// .map(|(page_i, page)| …)`).

### F17 — `let mut ser = …;` where `ser` is never mutated. *Read.*

`crates/meta-editor/src/main.rs:266` — serde's serializer API requires
`mut` for the `serialize` call, so technically this is correct, but
mut-bind-without-mutation trips clippy::unused_mut unless the func-call
mutates internally, which this one does. Minor.

### F18 — Debug-only `INDENT_DEBUG` env var dead code. *Read.*

`crates/miner/src/backend/mupdf.rs:120-127` — gated by both an env var
and `#[cfg(debug_assertions)]`. Works, but the same expression appears
twice (debug vs release arm). Inlined comment is from mupdf-history and
doesn't reflect the now-`oxidize` default.

### F19 — `#[allow(clippy::single_match)]` hides intent. *Read.*

`crates/miner/src/source.rs:82` — present on the same match that drops
all non-`RegexReplace` ops (F3). Couples the lint suppression to the
feature gap rather than the style.

### F20 — `#[allow(clippy::if_same_then_else, clippy::nonminimal_bool)]`
on the indented-tab logic. *Read.*

**File:** `crates/miner/src/backend/mupdf.rs:94` (**Read**)

The two `if-then-else` arms in `should_tab` collapse to plain booleans
(`if last_has_dot && !dot { false } else { true }` → `!(last_has_dot &&
!dot)`; `if dot { true } else { false }` → `dot`). They're emitted
opposite-branch only because of mupdf's original indentation control
flow. Cleans up the same way in either mupdf or oxidize.

### F21 — `HashMap<PathBuf, u64>` keyed off raw path, no canonicalization
on read. *Read.*

**File:** `crates/miner/tests/pdf_extract.rs:84` (**Read**)

`std::fs::canonicalize(entry.path()).unwrap()` is called on the cache
read path, but the cache is *written* from the example using
`std::fs::canonicalize(manifest_dir.join("../../pdf/")).unwrap()`
followed by walking; mismatched canonicalization between OSes or between
example and test will silently re-hash everything. Should canonicalize
once on write and once on read identically.

### F22 — `tests/pdf_extract.rs` reads only info-skeletons. *Read.*

**File:** `crates/miner/tests/pdf_extract.rs:69-75` (**Read**)

The seven test fixtures under `crates/miner/res/tests/data/*.json` are
all `"info": {...}` only. The `pdf_extract` example writes those
fixtures. With F1 broken, no fixtures ever get real contents, so the
test does not currently compare real extraction output to anything. It's
`#[ignore]`'d anyway (`tests/pdf_extract.rs:54`).

---

## 6. Repository hygiene findings (P2)

### F23 — Submodule URL is SSH-only. *Read.*

**File:** `.gitmodules:1-4` (**Read**)

```
[submodule "crates/miner/res/tests/data"]
    path = crates/miner/res/tests/data
    url = git@github.com:m00nwtchr/cofd-miner-test-data.git
    update = none
```

Contributors without GitHub SSH access cannot `git submodule update`. With
`update = none`, even if the URL is wrong, Git won't try to fix it.
Making the test-data submodule optional (or moving fixtures into the
main repo) would simplify onboarding.

### F24 — `pdf/` is a relative symlink to a non-checked-in path. *Read.*

```
pdf -> /home/m00n/Documents/RPG/CofD/Books
```

Will be a dangling symlink on any host that isn't the user's. Should be
gitignored (currently is — `.gitignore:5`) and documented in README.

### F25 — `.pre-commit-config.yaml` is a symlink to `/nix/store/`. *Read.*

Resolved to `/nix/store/vhqyg4r6c04ajh67frvidf6yvnn4af96-pre-commit-config.json`.
Re-emerges as a dangling symlink on any non-Nix host, and the comment
`# DO NOT MODIFY / generated by git-hooks.nix` makes this an unintentional
coupling.

### F26 — `llm/` directory is abandoned. *Read.*

**Path:** `llm/` (**Read**)

- `llm/llm.py` — hard-codes LLaMA-13B + transformers + torch, sample
  "Cold Embrace (Cunning)" gift, hand-written prompt; defines
  `generate_facet_json` twice (signature collision).
- `llm/llm.sh` — talks to Ollama at `localhost:11434` with a hard-coded
  `llama3.2:1b`.
- `llm/test.py` — spaCy-based NER experiment with custom labels that are
  never trained (`FACET_NAME`, `RENOWN`).

Nothing here is wired into Cargo. It's superseded by the regex-based
parsers. Move to a `archive/llm-poc/` (preserved for history) or delete.

### F27 — `codex-scraper` is the highest-complexity, lowest-stability code
in the workspace. *Verified.*

**Graph:** `cofd-miner.crates.codex-scraper.src.gifts.parse_gifts` —
`complexity=23`, `cognitive=94`. Second is `mupdf.extract_pages`
(`cognitive=54`). `codex-scraper.parse_merits` — `cognitive=30`.

Inside `codex-scraper/src/gifts.rs:92-95`:
```rust
// dice_pool: todo!(),
// action: todo!(),
// duration: todo!(),
```
These are commented `todo!()` *inside an `if` arm that is itself
commented out* but the surrounding `match` arm in `codex-scraper/src/lib.rs:88-95`
calls into the same data flow.

`codex-scraper/src/merits.rs:39-42`:
```rust
.unwrap_or_else(|er| {
    println!("{book}, {er}");
    panic!()
}),
```
Panics on the first unparseable row.

The wiki target `Merits,_Universal` and `Merits,_Vampire` is commented
out in `examples/scrape.rs:11-19`. The example uses `tokio::spawn` then
drops the JoinHandle via `for task in tasks { task.await? }`, which is
fine, but the work itself is fragile.

Recommendation: don't try to keep this alive unless actively scraping;
otherwise delete or move to `archive/`.

### F28 — `devenv.nix` pins a specific nightly date. *Read.*

**File:** `devenv.nix:43` (**Read**): `version = lib.mkForce "2026-01-01";`

While `rust-toolchain.toml` says `channel = "nightly"` (no date).
Coupling between two systems; bump or remove.

### F29 — Large commented-out block left in `oxidize.rs`. *Read.*

`crates/miner/src/backend/oxidize.rs:46-150` — 105 lines of commented
mupdf indent-reconstruction logic. Wastes screen real estate and creates
an asymmetry between git history and the working tree. Either delete or
restore.

### F30 — `pages_start.parse().unwrap()` etc. — F11 already covers this.

### F31 — `pdf-extract` writes into the submodule. *Read.*

`examples/pdf-extract.rs:63-66`:
```rust
let out_path = &manifest_dir.join("res/tests/data/");
if !out_path.exists() {
    std::fs::create_dir_all(out_path)?;
}
```

If the submodule isn't initialized, `create_dir_all` will create an
empty directory in its place, masking the broken state. The user has to
run `git submodule update --init` separately.

---

## 7. Knowledge-graph complexity & hotspot ranking

Verified via `codebase-memory-mcp_query_graph` against project
`cofd-miner`:

| Function                                                       | complexity | cognitive | loop_depth |
| -------------------------------------------------------------- | ---------: | --------: | ---------: |
| `codex-scraper::src::gifts::parse_gifts`                       |         23 |        94 |          2 |
| `miner::src::backend::mupdf::extract_pages`                    |         18 |        54 |          3 |
| `miner::src::source::process_section`                          |         11 |        29 |          1 |
| `codex-scraper::src::merits::parse_merits`                     |          9 |        30 |          2 |
| `miner::src::parse::merit::parse_merits`                       |          6 |        13 |          3 |
| `codex-scraper::src::lib::parse`                               |          6 |        11 |          5 |
| `miner::src::parse::gift::parse_gifts`                         |          4 |        10 |          3 |

Read this list as: the two "real" complexity hotspots are
`mupdf::extract_pages` and `process_section` — i.e. the two places that
fix F1, F2, F3 land. The four `codex-scraper` functions are vestigial
(F27). The two `miner::parse` functions are the ones that need F4, F5,
F6.

---

## 8. Verification matrix (this doc)

| Claim                                         | Status        | How verified                                               |
| --------------------------------------------- | ------------- | ---------------------------------------------------------- |
| F1 oxidize stub                               | **Verified**  | `get_code_snippet` on `oxidize.extract_pages`; graph complexity=1 |
| F2 mupdf not declared                         | **Verified**  | `backend/mod.rs` snippet; `Cargo.toml:11-26` direct read    |
| F3 only `RegexReplace` handled                | **Verified**  | `process_section` snippet                                  |
| F4 MageSpell dropped                          | **Verified**  | `PdfExtract::parse` snippet                                |
| F5 Moon gifts no-op                           | **Verified**  | `PdfExtract::parse` snippet                                |
| F6 `.unwrap()` on parse_merits                | **Verified**  | `PdfExtract::parse` snippet                                |
| F7 hash u64/hex mistake                       | **Retracted** | `cofd-rs` `BookInfo` snippet proves `#[serde(with="hex")]`  |
| F8 lopdf `PageText` undefined                 | **Verified**  | `lopdf.extract_pages` return type; `mod.rs` only has `PdfText` |
| F9 `from_slice(...).unwrap()` on META         | **Read**      | `lib.rs:30-32` direct read                                 |
| F10 meta-editor `Result` dropped              | **Read**      | `meta-editor/src/main.rs:265-271` direct read              |
| F11 meta-editor parse().unwrap()              | **Read**      | `meta-editor/src/main.rs:195,205` direct read              |
| F13 `let str = …` shadow                      | **Read**      | `grep` of `crates/**/*.rs`                                 |
| F14 leftover `println!`                       | **Read**      | `grep -rn "println\|dbg!" crates`                          |
| F15 duplicated walker/Cache                   | **Verified**  | graph topology shows three same-shape clusters + file reads |
| F22 fixtures are info-only                    | **Read**      | `cat res/tests/data/*.json`                                |
| F23 SSH-only submodule                        | **Read**      | `.gitmodules`                                              |
| F24 `pdf` symlink                             | **Read**      | `ls -la pdf`                                               |
| F26 `llm/` abandoned                          | **Read**      | file reads                                                 |
| F27 `codex-scraper` broken                    | **Verified**  | graph `complexity`/`cognitive` ranking + `todo!`/`panic!`   |

---

## 8a. Stable-Rust build verification

Verified on Rust 1.96.0 stable (from `/nix/store/vcmnaxr6…`) by
putting the stable `cargo` and `rustc` ahead of nightly in `PATH`:

```
$ cargo check --workspace --offline
...
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> …/cofd/util/src/lib.rs:1:1
  |
1 | #![feature(generic_const_exprs)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
error: could not compile `cofd-util` (lib) due to 1 previous error
```

The single block is `cofd-util/src/lib.rs:1` — `#![feature(generic_const_exprs)]`.
`cofd-util` is a transitive dep via `cofd-schema`. Required for
`fn all() -> [Self; Self::N]` on `pub trait AllVariants` in the `util`
sub-crate of the `cofd` workspace at pinned commit
`89b07a2cfb7d11b9a829f831c97baf63246aac8e`.

### With the blocker resolved (shim test)

To verify everything *else* is stable-compatible, I patched out
`cofd-util` and `cofd-derive` to local shims (`[patch.git]` in
`.cargo/config.toml`, dropped on completion) with:

- `edition = "2024"` and `proc-macro = true` on `cofd-derive`
- `edition = "2024"` and the `Self::N` array replaced by `Vec<Self>`
  on `cofd-util`

Result:

```
$ cargo check --workspace --all-targets --offline
…
warning: `cofd-miner` (lib) generated 8 warnings (4 duplicates) …
warning: `cofd-miner` (lib test) generated 8 warnings (4 duplicates) …
warning: `cofd-miner` (test "pdf_extract") generated 3 warnings …
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.40s
```

**The entire workspace (lib + tests + examples + bins, all 4 crates)
compiles cleanly on Rust 1.96.0 stable with the same warnings as on
nightly.** No new errors or warnings are introduced by switching to
stable once the upstream dep is patched.

### Conclusion

| Variant                          | Stable 1.96.0 | Nightly 1.94.0 |
| -------------------------------- | ------------- | -------------- |
| as shipped (locked `cofd@89b07a2`) | **FAIL** (`cofd-util` `generic_const_exprs`) | OK, 8 warnings |
| with `cofd-util` shimmed          | OK, 8 warnings (same as nightly) | n/a |

### F32 (new finding) — stable-Rust build is broken by a transitive dep.

**Category:** P0 (blocks a stable release channel option)
**Files:** `~/.local/share/cargo/git/checkouts/cofd-35b7feef086ad1de/89b07a2/util/src/lib.rs:1`
**Verified:** reproduced on stable 1.96.0 in an offline `--check`.

Recommendation (priority list):

1. Switch the `cofd` git dep to a more recent commit where
   `cofd-util` no longer requires `generic_const_exprs` (or
   re-implements the trait using `Vec<Self>`-returning or
   compile-time-known `N`), so MSRV can include stable 1.96+.
2. If staying on commit `89b07a2c` is unavoidable, fork `cofd-util`
   into the workspace under `crates/cofd-util-shim/`, remove the
   `#![feature]` attribute, replace the const-expr array with
   `Vec<Self>`, and add a `[patch.git]` for `cofd-util`. The simpler
   path is to just bump the `cofd` dep to a newer commit.
3. Drop `rust-toolchain.toml`'s `channel = "nightly"` once this is
   fixed; document an MSRV (probably 1.85 for `edition = "2024"`).

`rust-toolchain.toml:2` currently pins the project to nightly; the
MSRV story is not documented anywhere visible.

---

## 9. Recommended sequencing (no code changes in this audit)

The fix list, ordered for delivery:

**P0 — extractor functionality**

1. Pick a backend winner. Either restore `mupdf` (full implementation
   already exists) or finish porting the indent logic into `oxidize`.
   The other should be removed.
2. Implement `Op::Insert`, `Op::Delete`, `Op::Move`, `Op::Replace` in
   `process_section`. (Pay particular attention to pos indices being
   against `original` rather than `extract`.)
3. Implement `PageKind::MageSpell` parsing in `parse/mod.rs` (mirror
   what `parse_gifts` does).
4. Implement `GiftKind::Moon` parsing (lift the `Moon` arm from
   `codex-scraper`).
5. Replace `.unwrap()` on `parse_merits` with `?` and propagate.

**P1 — robustness**

6. Add `#[serde(with = "hex")]`-aware hash matching test against a
   real fixture, even a small one, in CI.
7. `expect("meta.bin decode failed: see build.rs logs")` on the
   embedded META decode so failures are debuggable.
8. `meta-editor`: surface save failures, validate page-number inputs,
   remove `extract_pages().unwrap()`.
9. Fix or delete `lopdf.rs`.

**P2 — cleanup**

10. Remove `let str = ` shadows (six files).
11. Delete `llm/`, or move to `archive/llm-poc/`.
12. Switch the test-data submodule URL to HTTPS or move fixtures into
    the main repo.
13. Remove the `pdf` symlink; document `pdf/` in README instead.
14. Delete the commented-out mupdf port from `oxidize.rs` (105 lines).
15. Decouple `devenv.nix` nightly date from `rust-toolchain.toml`.

---

## 10. Audit metadata

- Auditor: opencode agent (model `minimax/MiniMax-M3`)
- Working tree: dirty (`git status` shows 20 modified + 1 untracked:
  `crates/miner/src/backend/oxidize.rs`)
- Branch: `master` @ `b82412f6be17ab21bd3bca587040700338d5dfdf`
- Toolchain pinned: `nightly` (toolchain file); `2026-01-01` (devenv.nix)
- Files audited by direct read: 14
- Files audited by graph `get_code_snippet`: 7
- Graph queries run: 4 (architecture, search_graph, get_code_snippet
  ×3, query_graph ×2)
- No code changes made.
