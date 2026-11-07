# CofD Foundry

The [CofD](https://github.com/m00nwtchr/cofd) rules engine, packaged as a Foundry
Virtual Tabletop module.

The module ships a raw WebAssembly build of the `cofd` Rust crate. It does not
need wasm-bindgen-generated glue or the `wasm-pack`/`wasm-bindgen` CLI: the small
`scripts/cofd-foundry.js` loader fetches `cofd_foundry.wasm` next to the module
and instantiates it with an empty import object.

## Installation

Copy the contents of `cofd-foundry.zip` into your Foundry `Data/modules/`
directory, or point Foundry at the manifest of a development build:

```
Data/modules/cofd-foundry/module.json
```

The module registers no compendium, sheet, or macro; it only loads the rules
engine and exposes it on `globalThis`.

## Usage

```js
const api = await globalThis.cofdFoundryWasmReady;
console.log(api.version, api.templateCount, api.mortalCharacterSize);
```

`globalThis.cofdFoundry` is available immediately and is filled in as soon as the
WebAssembly module has been instantiated:

| Field                        | Meaning                                            |
| ---------------------------- | -------------------------------------------------- |
| `version`                    | Version of the embedded `cofd` crate               |
| `templateCount`              | Number of character templates known to `cofd`      |
| `mortalCharacterSize`        | Size in bytes of a mortal character in memory      |
| `ready`                      | `true` once the WebAssembly module is instantiated |
| `error`                      | The error that prevented loading, if any           |
| `instance`                   | The raw `WebAssembly.Instance`                     |

A smoke message is written to the console when Foundry fires its `init` hook.

## Building

From the repository root:

```
crates/cofd-foundry/build-module.sh
```

This compiles the crate for `wasm32-unknown-unknown`, stages the module in
`crates/cofd-foundry/dist/cofd-foundry/`, and writes
`crates/cofd-foundry/dist/cofd-foundry/cofd-foundry.zip`.
