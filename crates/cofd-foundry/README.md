# cofd-foundry

Foundry Virtual Tabletop packaging for the [`cofd`](../../crates/cofd) rules
engine. The crate compiles to WebAssembly and uses UniFFI plus
`uniffi-bindgen-react-native` (`ubrn`) to generate the browser bindings consumed
by Foundry.

## Layout

| Path                 | Purpose                                             |
| -------------------- | --------------------------------------------------- |
| `src/lib.rs`         | The crate and UniFFI scaffolding                    |
| `Cargo.toml`         | Crate and Foundry system metadata                   |
| `system/cofd.ts`     | WASM binding consumer and character sheet          |
| `system/templates/`  | Handlebars templates for the system sheets          |
| `build-system.sh`    | Builds the WASM and stages the distributable system |
| `ubrn.config.yaml`   | UBRN build configuration                            |
| `package.json`       | Pinned UBRN build dependencies                      |
| `dist/`              | Build output (git-ignored)                          |

## WebAssembly bindings

The Rust crate uses UniFFI `0.31` exports for compatibility with the pinned UBRN
release. `build-system.sh` runs `ubrn build wasm2` against the compiled WASM,
then uses `tsdown` to bundle the generated TypeScript and the authored system
loader into `cofd.js`. The stage also contains the generated TypeScript sources,
FFI signature tables, and `cofd_foundry.wasm`.

| Export                   | Purpose                                      |
| ------------------------ | -------------------------------------------- |
| `version`                | Version of the embedded `cofd` crate        |
| `listTemplates`          | Enumerates schema templates                  |
| `listSupportedTemplates` | Enumerates constructible templates           |
| `listAttributes`         | Enumerates schema attributes                 |
| `listSkills`             | Enumerates schema skills                     |
| `listDerivedTraits`      | Enumerates derived traits                    |
| `schemaEnums`            | Values for generated Rust schema enums       |
| `CharacterHandle`        | Engine-backed character object               |
| `WeaponHandle`           | Character weapon value                       |

The system loader initializes the generated UBRN entrypoint before using the
engine. UBRN objects are constructed with `new` and can be explicitly released
with `uniffiDestroy`:

```js
const api = await globalThis.cofdFoundryWasmReady;
const character = new api.CharacterHandle("Mortal");
character.setTrait("Stamina", 3);
character.setTrait("Size", 5);
console.log(api.version, character.getTrait("Health"));
character.uniffiDestroy();
```

`CharacterHandle` exposes engine-backed trait reads and writes, derived values,
damage and wound operations (`applyDamage` fills one box per point and upgrades
lesser damage only when the track is full; `clearDamage` removes boxes while
`healDamage` downgrades severity), character info, specialties, collections,
weapons, werewolf forms, and supported abilities. `weaponAt(index)` returns a
copy; use `setWeapon` to write edits back. Unsupported CofD templates return an
error instead of entering an unfinished engine path.

## Building

From the repository root:

```
pnpm --dir crates/cofd-foundry install --frozen-lockfile
crates/cofd-foundry/build-system.sh
```

`ubrn.config.yaml` points UBRN at the workspace crate and stages its `wasm2`
output in `dist/cofd/`. The generated `index.ts` exports `uniffiInitAsync`, and
the authored TypeScript loader calls it with the adjacent WASM asset before
exposing the Foundry API.

The script copies generated bindings and static system files into `dist/cofd/`,
bundles `system/cofd.ts` as `cofd.js`, generates `dist/cofd/system.json` from
`[package.metadata.foundry]` and the Cargo package version, and writes
`dist/cofd/cofd.zip` with the Python standard library. UBRN and its
`@ubjs/core`/`@ubjs/wasm` runtime packages, plus `tsdown`, are pinned in
`pnpm-lock.yaml`.

## Checking

```
cargo check -p cofd-foundry
cargo test -p cofd-foundry
pnpm --dir crates/cofd-foundry run test:bundle
```
