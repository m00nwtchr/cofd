# cofd Codebase Findings

Verification date: 2026-07-24
Toolchain context: Nightly → Stable switch applied (verified via codebase MCP)

All findings below were validated against the codebase knowledge graph
(`codebase-memory-mcp`). Source-of-truth line numbers come from the graph's
extracted source snippets, not manual line counts.

---

## 1. Critical Bugs (correctness-affecting)

### 1.1 `Vampire::template()` returns the wrong template

- **File:** `lib/src/splat/vampire.rs:51-53`
- **Source (verified):**
  ```rust
  fn template(&self) -> Template {
      Template::Hunter
  }
  ```
- **Should be:** `Template::Vampire`
- **Impact:** `SplatCharacter::Vampire(...)` attributes are built against the
  Vampire template via `splat_attributes(template)` (called from
  `Character::new`), so this defect silently produces a Vampire character
  with the wrong attribute map (Hunter's lack of any splat-specific
  additions and any later `BaseSkill` bonuses are mixed in instead).

### 1.2 Proc-macros look for the wrong data file format and location

- **File:** `macros/src/lib.rs`
- **Verified source (line 37-45):**
  ```rust
  let path = Path::new(&env::var("CARGO_MANIFEST_DIR").expect(""))
      .join("data")
      .join("Gifts.json");
  let book: Book = if path.exists() {
      facet_json::from_reader(File::open(path).expect("")).expect("Parsing error")
  } else {
      Book::default()
  };
  ```
- **`merits!()` (line 164-172):** Same pattern, reads `merits_universal.json`.
- **Issue:**
  1. The actual files are `.ron` (`lib/data/Gifts.ron`,
     `lib/data/merits_universal.ron`).
  2. `CARGO_MANIFEST_DIR` resolves to the `cofd-macros` crate directory, but
     `data/` lives in `cofd` (`lib/data`).
  3. Even if the JSON-extant data existed at the path, the file would still
     not be found because `cofd-macros/data` does not exist.
- **Impact:** `gifts!()` and `merits!()` always evaluate to empty
  `Book::default()`. The `MoonGift`, `ShadowGift`, `WolfGift`, and `Facet`
  enums emitted from the macro are empty (just a `Custom` variant). The
  `merits!()` macro body is also a no-op (`for merit in book.merits {}`).
  See `macros/src/lib.rs:173`.

### 1.3 `gifts!()` macro declares duplicate variants when facets overlap

- **File:** `macros/src/lib.rs:55-100`
- **Verified:** Two `facet_variants.extend(...)` call sites —
  - Line 63: inside the `book.moon_gifts` loop
  - Line 86: inside the `book.gifts` loop
- **Impact:** Any facet name that appears in both a `MoonGift` and a
  `ShadowGift`/`WolfGift` will produce a duplicate enum variant declaration
  in the generated `Facet` enum, which is a compile error. The two
  source-data loops are deliberately separated to avoid exactly this, but
  the deduplication was never added.

### 1.4 `Seeming::favored_attributes()` returns a placeholder

- **File:** `schema/src/template/changeling.rs:42-58`
- **Verified source:**
  ```rust
  pub fn favored_attributes(&self) -> [Attribute; 3] {
      // Attribute::get(AttributeCategory::Type(match self { ... }))
      // TODO
      [
          Attribute::Intelligence,
          Attribute::Intelligence,
          Attribute::Intelligence,
      ]
  }
  ```
- **Impact:** `Changeling::default()` (`lib/src/splat/changeling.rs:135`)
  reads `seeming.favored_attributes()[0]` to seed `attr_bonus`. Every
  Changeling starts with `Attribute::Intelligence` as its favored
  attribute — silently wrong.

### 1.5 `wound_penalty()` match arms invert the lookup

- **File:** `lib/src/character/mod.rs:134-145`
- **Verified source:**
  ```rust
  match mh - self.health.total().min(mh) {
      2 => 1,
      1 => 2,
      0 => 3,
      _ => 0,
  }
  ```
- **Impact:** `mh - total().min(mh)` yields the number of *empty* boxes
  (mh − min(total, mh)). When all boxes are empty the expression yields 0,
  yet the function returns 3 — the worst penalty bracket. Per CofD rules
  "no boxes checked ⇒ no penalty"; this code does the opposite.
  Additionally the mapping (`2→1`, `1→2`, `0→3`) is the inverse of the
  CoFD penalty table (which is `1→1, 2→2, 3→5, ...`).

### 1.6 `splat_attribute_builder` panics for every non-Werewolf template

- **File:** `lib/src/lib.rs:201-235`
- **Verified source (last line of the `match`):** `_ => todo!()`
- **Triggered via:** `splat_attributes(template)` → `Character::new()` →
  `Actor::new(kind)`. This is the entry point used by
  `SplatCharacter::new()` (`lib/src/splat/mod.rs:74-86`) for every splat
  *other* than Werewolf.

### 1.7 `SplatCharacter::new()` panics on unhandled templates

- **File:** `lib/src/splat/mod.rs:74-86`
- **Verified source:**
  ```rust
  match kind {
      Template::Mortal => ...,
      Template::Vampire => ...,
      Template::Werewolf => ...,
      Template::Mage => ...,
      Template::Changeling => ...,
      Template::Bound => ...,
      _ => unimplemented!("Character splat not yet implemented"),
  }
  ```
- **Impact:** `Template::Promethean`, `Template::Hunter`,
  `Template::Mummy`, `Template::Demon`, `Template::Beast`,
  `Template::Deviant`, `Template::WolfBlooded`, `Template::FaeTouched`,
  etc. — all panic at `SplatCharacter::new()`. Note also that
  `Template::WolfBlooded` and friends have no `SplatCharacter` variant
  (cf. `mod.rs:55-68`) so they would need to share a fallback splat.

### 1.8 `Merit::all()`, `Kith::all()`, `Court::all()`, etc. fail at compile time

- **File:** `lib/src/splat/changeling.rs`, `lib/src/splat/vampire.rs`, etc.
- **Issue:** The code calls `.all()` on enums to enumerate variants, but:
  - The `AllVariants` derive macro is **fully commented out** in
    `derive/src/lib.rs:260-373`.
  - The `AllVariants` trait definition is **fully commented out** in
    `util/src/lib.rs:29-37` (and only the bound `for<'a> facet::Facet<'a>`
    body `todo!()` survives).
- **Impact:** `Mortal::merits() → Merit::all()` (line 34),
  `Changeling::merits() → ChangelingMerit::all()`,
  `Bound::ysplats() → Archetype::all()`, `Vampire::ysplats() → Covenant::all()`,
  `Vampire::zsplats() → Bloodline::all()`, `Vampire::all_abilities() → Discipline::all()`,
  `Bound::all_abilities() → Haunt::all()`, etc. all refer to a trait that
  no longer exists. These crates fail to compile until either the derive is
  restored or the trait is reconstituted.

### 1.9 Missing `#[repr(u8)]` on facet-derived enums

- **Files:** multiple, wherever the macro is called.
- **Evidence (from the build verification):**
  > `error: Facet requires enums to have an explicit representation (e.g., #[repr(C)], #[repr(u8)])`
- **Most affected enums:**
  - Any enum produced by the `gifts!()` macro (the macro itself supplies
    `#[repr(u8)]`, but the generated types live in the macro expansion
    scope).
  - Several hand-written enums in `lib/src/splat/*` lack the annotation.

### 1.10 Missing `Serialize`/`Deserialize` on `facet::Facet` struct types

- **Files:** `schema/src/book.rs`, `schema/src/item/*.rs` with the
  `cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))`
  attribute on **enums** but not on the surrounding structs (e.g.
  `Item<T>`, `Book`, `Gift<T>`, `Facet<T>`).
- **Evidence (build output, with `--features json_schema,serde`):**
  > `the trait bound 'item::Item<Merit>: serde::Serialize' is not satisfied`
  > `the trait bound 'Gift<Moon>: serde::Serialize' is not satisfied`

---

## 2. Architectural Issues

### 2.1 Massive dead-code mass (~600+ lines commented out)

| File | State |
|---|---|
| `lib/src/character/modifier.rs` | 442 lines; ~99% commented |
| `lib/src/character/damage.rs` part | parts commented |
| `lib/src/traits.rs:9-39` | huge commented `Trait` enum |
| `lib/src/merits.rs:191-433` | ~240 lines commented |
| `lib/src/dice_pool.rs:14-31` | `value()` impl commented |
| `lib/src/splat/vampire.rs:241-475` | mostly commented |
| `lib/src/splat/werewolf.rs:584-891` | huge commented sections |
| `derive/src/lib.rs:172-373` | `VariantName` + `AllVariants` derives entirely commented |
| `util/src/lib.rs:5-37` | entire content commented |
| `lib/tests/test.rs:127-375` | the bulk of the integration test commented |
| `macros/src/lib.rs:175` | empty token stream — `merits!()` no-op |
| `lib/src/lib.rs:215-220, :231` etc. | defensive dead helpers |

The dead code is historically interesting but actively dangerous: it
references symbols that no longer exist (the legacy modifier system), and
callers in commented code cannot be detected by the type checker.

### 2.2 Inconsistent `Custom` variant patterns across splat enums

| Enum | Pattern |
|---|---|
| `Clan` | `Custom { name, disciplines, favored_attributes }` (struct) |
| `Auspice` | `Custom(CustomAuspice)` (tuple) |
| `Path` (Mage) | `Custom { name, ruling_arcana, inferior_arcanum }` (struct) |
| `Seeming` | `Custom { name, regalia }` (struct) |
| `Burden` | `Custom { name, haunts }` (struct) |
| `Court`, `Kith`, `Covenant`, `Discipline`, `Archetype`, `HuntersAspect` | `Custom(String)` (tuple with single String) |
| `Bloodline` | `Custom(String, Option<[Discipline; 4]>)` (tuple) |
| `Lodge` | not an enum; struct `{ name: String }` |

- **Impact:** `XSplat::name_mut()` and `is_custom()` (`lib/src/splat/mod.rs:208-287`)
  must hand-code matching for each splat — `Auspice::Custom(CustomAuspice { name, .. })`
  vs `Clan::Custom { name, .. }` vs `Court::Custom(name)`. Forgetting one
  variant silently breaks `is_custom()` callers. There is no compile-time
  check that the lists are exhaustive.

### 2.3 Inconsistent `is_custom()` / `name_mut()` enumeration

- **File:** `lib/src/ability.rs:32-58` (`Ability::name_mut`/`is_custom`),
  `lib/src/splat/mod.rs:208-287`.
- **Issue:** the methods enumerate `Custom` variants manually. `Arcanum`,
  `Renown`, `Haunt`, `ChangelingMerit` are not handled in
  `Ability::name_mut` even though those types have `Custom` variants that
  could appear via the variant tuple variants `Ability::Arcanum`,
  `Ability::Haunt`, etc.
- **Impact:** A `Custom` Arcanum / Haunt cannot currently be edited
  through `name_mut()`.

### 2.4 `Merit` enum has duplicate `From` impls

- **File:** `lib/src/merits.rs:13-188`
- **Pattern:** `Merit` derives `derive_more::From` AND has per-variant
  `#[from]` attributes on `Mage(MageMerit)`, `Vampire(VampireMerit)`,
  `Werewolf(WerewolfMerit)`, `Changeling(ChangelingMerit)`.
- **Impact:** `derive_more::From` will auto-derive `From<MageMerit> for Merit`,
  `From<VampireMerit> for Merit`, etc. The `#[from]` attributes then
  duplicate the same impls. With `derive_more = "2"` this typically
  produces a duplicate-impl compile error or, on newer versions, a
  diagnostic.

### 2.5 `WerewolfMerit::merits()` uses runtime facet introspection

- **File:** `lib/src/splat/werewolf.rs:230-251`
- **Verified:** Uses `facet::peek_enum_variants`,
  `Partial::alloc::<WerewolfMerit>()`, `select_variant_named`,
  `set_nth_field_to_default`, `build`, `materialize`. This dynamically
  synthesizes every variant of `WerewolfMerit` with all fields set to
  `Default::default()`.
- **Impact:** Works today only because every variant field is
  `Default`-constructible. Any future variant that holds a non-Default
  type (e.g. `String` without default fill) will silently produce a
  half-initialised value.

### 2.6 Trait dispatch through dead helpers

- `defense_pool` (`lib/src/character/mod.rs:24-26`) is defined but never
  used.
- The entire `Modifiers` system in `lib/src/character/modifier.rs` is
  commented out, yet call sites (also commented) reach for it. Reverting
  to a working non-`systema` modifier model would require re-introducing
  this file.

### 2.7 Redundant `extern crate` in edition 2024

- **File:** `lib/src/lib.rs:3-4`
  ```rust
  #[macro_use]
  extern crate cofd_util;
  ```
  Unnecessary in edition 2024 (`use cofd_util::AllVariants;` at line 39
  is the only thing actually being relied on).

### 2.8 Pre-existing dead proc-macro

- `extend_enum` (`macros/src/lib.rs:182-209`) is defined but has zero
  callers in the codebase (graph `out_degree = 1` — only to
  `derive_more::Debug`).

---

## 3. Build-Time / Dependency Issues

### 3.1 `itertools = "*"` wildcard

- **File:** `schema/Cargo.toml:23`
- **Risk:** A wildcard lets Cargo pull any future version, including one
  that may require nightly. Today the lockfile pins `0.14.0`
  (`rust-version = "1.63"`) which is stable, but there is no guarantee.
- **Fix:** Pin to `"0.14"`.

### 3.2 `rust-toolchain.toml` → nightlies behind the scenes

- **Original file:** `channel = "nightly"`, `profile = "minimal"`.
- **Fixed:** replaced with `channel = "stable"` (verified via MCP — `toolchain`
  Class node source is now `channel = "stable"\nprofile = "minimal"`).
- **Side note:** `devenv.nix:29` already declared `channel = "stable"`,
  but cargo + rust-toolchain.toml outranked devenv at run time. After
  the fix, devenv's stable channel and the toolchain file agree.

### 3.3 Toolchain pinning best-practice (advisory)

- No `rust-version` is set in `Cargo.toml`. Adding
  `rust-version = "1.85"` to the workspace `[workspace.package]` block
  would document the minimum supported version explicitly.

### 3.4 `facet-json` is a streaming parser

- The proc-macro uses `facet_json::from_reader`. If the data format ever
  changes from JSON to RON, the macros need to be switched accordingly
  (cf. Section 1.2 — the actual files are RON).

### 3.5 Untracked `devenv.lock` change

- `git status` shows `devenv.lock` is modified but unstaged. The diff
  removes `git-hooks` and `flake-compat` inputs, suggesting an
  intentional removal of a devenv sub-feature.

---

## 4. Linter / Style Issues

### 4.1 `#![warn(clippy::pedantic)]` enabled unconditionally

- **Files:** `lib/src/lib.rs:1`, `schema/src/lib.rs:1`.
- **Pattern:** Combined with `#[allow(...)]` scattered throughout (mostly
  the `clippy::too_many_lines`, `clippy::wildcard_imports` clusters),
  pedantic lints operate inconsistently across the workspace.
- **Recommendation:** either commit to pedantic (with full
  allowance-rationale), or drop it to default.

### 4.2 Unused imports in 16 modules

- 16 files have `use serde::{Deserialize, Serialize};` while only some
  types actually use one or both derives. Examples:
  - `lib/src/splat/mage.rs:3` — neither `Deserialize` nor `Serialize`
    is used.
  - `lib/src/splat/geist.rs:2` — same.
  - `lib/src/splat/vampire.rs:7` — `Attribute` import unused.
  - `lib/src/splat/werewolf.rs:7` — `HuntersAspect::Monstrous` import
    used only in commented-out code.
- **Other examples:** `lib/src/splat/changeling.rs:3`,
  `lib/src/character/damage.rs:3`, `lib/src/character/modifier.rs:7`,
  `lib/src/lib.rs:11`, `lib/src/merits.rs:2`, etc.

### 4.3 `Unused variable: character` warnings

- `lib/src/dice_pool.rs:10`, `lib/src/splat/mage.rs:228`,
  `lib/src/splat/changeling.rs:219` — `character` parameter is unused
  in `is_available()` methods.
- Several default arms in `splat/mod.rs` (lines 143-159) accept `name`
  but never use it.

---

## 5. Crate `cofd` Build Status on Stable (1.95.0)

Verified via `rustc 1.95.0` from the project's stable toolchain:

| Crate | `cargo check --lib` | Result |
|---|---|---|
| `cofd-schema` | OK (2 unused-import warnings) | Compiles |
| `cofd-util` | OK | Compiles |
| `cofd-derive` | OK (3 unused-import warnings) | Compiles |
| `cofd-macros` | OK (2 unused-import warnings) | Compiles |
| `cofd` (lib) | 39 errors | Pre-existing source bugs (see §1) |

39 errors in `cofd` lib cluster around:
- Missing `#[repr(u8)]` on facet-derived enums (4× `error: Facet requires`).
- Unresolved `cofd_util::AllVariants` / `VariantName` imports (2×).
- `<Enum>: Iterator` not satisfied — caused by `.all()` on enums lacking
  `VariantArray` or the (defunct) `AllVariants` derive (~20×).
- `Ability: serde::Serialize` not satisfied (trait bounds).
- `AttributeMap<...>: facet::Facet<'_>` not satisfied (systema map lacks
  `Facet` derive).
- `<Merit>: facet::Facet<'_>` not satisfied (the derive enum struct).

None of these errors are caused by the toolchain switch — they existed
before and would also surface on nightly. They are the natural next set
of fixes.

---

## 6. Recommended Fix Order

Priorities for follow-up work (no code changes performed):

1. **§1.1** `Vampire::template` → `Template::Vampire` (1-line, high impact)
2. **§1.4** Implement `Seeming::favored_attributes` properly
3. **§1.5** Fix `wound_penalty()` (CoFD-accurate table)
4. **§1.6–1.7** Replace `_ => todo!()` / `unimplemented!()` in the splat
   builders with concrete arms for at least Mortal / Vampire / Mage /
   Changeling / Bound
5. **§1.2** Move data to `cofd-macros/data/` (or convert to inline `include_str!`)
   and switch the parser to `ron`
6. **§1.3** Deduplicate `facet_variants` in `gifts!()`
7. **§1.8** Restore the `AllVariants` derive macro (either uncomment or
   rewrite to use `strum::VariantArray`)
8. **§1.9–1.10** Add `#[repr(u8)]` and `serde` derives where facet requires
9. **§2.4** Decide on `Merit::From` strategy (drop derive or drop `#[from]`)
10. **§2.5** Reimplement `WerewolfMerit::merits()` without runtime
    introspection (derive `VariantArray` + manual list)
11. **§3.1** Pin `itertools = "0.14"`
12. **§3.3** Add `rust-version = "1.85"` to workspace
13. **§4.1–4.3** Lint cleanup (clippy + unused imports)
14. **§2.1** Delete (or restore) commented-out code en masse

---

## 7. Cross-Reference Table (graph-verified)

| Finding | Graph node | Line range |
|---|---|---|
| §1.1 | `lib.src.splat.vampire.Vampire.template` | 51-53 |
| §1.2 | `macros.src.lib.gifts`, `macros.src.lib.merits` | 36-160, 163-178 |
| §1.3 | `macros.src.lib.gifts` facet-extension sites | 63, 86 |
| §1.4 | `schema.src.template.changeling.Seeming.favored_attributes` | 42-58 |
| §1.5 | `lib.src.character.mod.Character.wound_penalty` | 134-145 |
| §1.6 | `lib.src.lib.splat_attribute_builder` | 201-235 |
| §1.7 | `lib.src.splat.mod.SplatCharacter.new` | 74-86 |
| §1.8 | `derive.src.lib` (AllVariants derive fully commented) | 260-373 |
| §1.8 | `util.src.lib` (trait def fully commented) | 1-39 |
| §2.1 | `derive.src.lib` (VariantName derive fully commented) | 172-258 |
| §2.1 | `lib.src.character.modifier` (entire file) | 1-443 |
| §2.8 | `macros.src.lib.extend_enum` | 182-209 (0 callers) |

All numbers above are from `codebase-memory-mcp` source extraction.
