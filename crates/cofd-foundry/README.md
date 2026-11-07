# cofd-foundry

Foundry Virtual Tabletop packaging for the [`cofd`](../../crates/cofd) rules
engine. The crate compiles to WebAssembly and uses `wasm-bindgen` to generate
the browser bindings consumed by Foundry.

## Layout

| Path                | Purpose                                                       |
| ------------------- | ------------------------------------------------------------- |
| `src/lib.rs`        | The crate and the wasm-bindgen exports                           |
| `system/system.json`| Foundry system manifest                                        |
| `system/cofd.js`    | WASM binding consumer and character sheet registration          |
| `system/templates/`| Handlebars templates for the system sheets                      |
| `build-system.sh`   | Builds the wasm and stages the distributable system           |
| `dist/`             | Build output (git-ignored)                                    |

## WebAssembly bindings

The Rust crate uses `wasm-bindgen` `0.2.126` exports. `build-system.sh` runs
the matching `wasm-bindgen` CLI with `--target web`, producing `cofd_foundry.js` and
`cofd_foundry_bg.wasm` next to the system loader:

| Export                 | Purpose                                      |
| ---------------------- | -------------------------------------------- |
| `version`              | Version of the embedded `cofd` crate         |
| `templateCount`        | Number of character templates                 |
| `mortalCharacterSize`  | Size of a mortal character in memory         |
| `mortalHealth`         | Engine-derived Health for stamina and size   |
| `mortalWillpower`      | Engine-derived Willpower for Resolve and Composure |

The system loader imports the generated module and initializes it normally:

```js
import init, { mortalHealth } from "./cofd_foundry.js";

await init();
console.log(mortalHealth(1, 5));
```

## Building

```
./build-system.sh
```

The script runs

```
cargo build -p cofd-foundry --target wasm32-unknown-unknown --release --locked
wasm-bindgen target/wasm32-unknown-unknown/release/cofd_foundry.wasm \
  --target web --out-dir dist/cofd --out-name cofd_foundry
```

copies the generated binding files and static system files into `dist/cofd/`,
and writes `dist/cofd/cofd.zip` with the
Python standard library.

## Checking

```
cargo check -p cofd-foundry
cargo test -p cofd-foundry
```
