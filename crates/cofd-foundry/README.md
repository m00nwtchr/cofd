# cofd-foundry

Foundry Virtual Tabletop packaging for the [`cofd`](../../crates/cofd) rules
engine. The crate compiles to a raw WebAssembly module — no wasm-bindgen glue,
no `wasm-pack`, no external toolchain — that Foundry loads directly.

## Layout

| Path                | Purpose                                                       |
| ------------------- | ------------------------------------------------------------- |
| `src/lib.rs`        | The crate and the raw WebAssembly exports                      |
| `module/module.json`| Foundry module manifest                                        |
| `module/scripts/`   | The ES module Foundry loads (`scripts/cofd-foundry.js`)        |
| `build-module.sh`   | Builds the wasm and stages the distributable module           |
| `dist/`             | Build output (git-ignored)                                    |

## WebAssembly ABI

The exports are gated to `wasm32` builds and use a plain C ABI:

| Export                                | Returns                                   |
| ------------------------------------- | ----------------------------------------- |
| `cofd_foundry_version_ptr`            | Pointer to the UTF-8 version string       |
| `cofd_foundry_version_len`            | Length of the version string in bytes     |
| `cofd_foundry_template_count`         | Number of character templates in `cofd`   |
| `cofd_foundry_mortal_character_size`  | Size of a mortal character in bytes       |

The module imports nothing, so it is instantiated with an empty import object:

```js
const { instance } = await WebAssembly.instantiate(bytes, {});
```

## Building

```
./build-module.sh
```

The script runs

```
cargo build -p cofd-foundry --target wasm32-unknown-unknown --release
```

copies `cofd_foundry.wasm` and the static module files into
`dist/cofd-foundry/`, and writes `dist/cofd-foundry/cofd-foundry.zip` with the
Python standard library.

## Checking

```
cargo check -p cofd-foundry
cargo test -p cofd-foundry
```
