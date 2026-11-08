# CofD Foundry

The [CofD](https://github.com/m00nwtchr/cofd) rules engine, packaged as a Foundry
Virtual Tabletop system.

The system ships a WebAssembly build of the `cofd` Rust crate with JavaScript
bindings generated from UniFFI TypeScript by `uniffi-bindgen-react-native` and
`tsdown`. The `cofd.js` bundle initializes the generated UBRN entrypoint and
registers the character sheet.

## Installation

Copy the contents of `cofd.zip` into your Foundry `Data/systems/` directory, or
point Foundry at the manifest of a development build:

```
Data/systems/cofd/system.json
```

The system registers a basic `character` Actor subtype and a default character
sheet. The sheet stores editable character data in the Actor's `system` field
and asks the embedded engine for derived mortal Health and Willpower values.

## Usage

```js
const api = await globalThis.cofdFoundryWasmReady;
const character = new api.CharacterHandle("Mortal");
character.setTrait("Stamina", 3);
character.setTrait("Size", 5);
console.log(api.version, character.getTrait("Health"));
character.uniffiDestroy();
```

`globalThis.cofdFoundry` is available immediately and is filled in as soon as the
WebAssembly module has been instantiated:

| Field                   | Meaning                                      |
| ----------------------- | -------------------------------------------- |
| `id`                    | Foundry system identifier                    |
| `CharacterHandle`       | UniFFI object constructor                    |
| `listTemplates`         | All schema templates                         |
| `listSupportedTemplates`| Constructible templates                      |
| `listAttributes`        | Schema attributes                            |
| `listSkills`            | Schema skills                                |
| `listDerivedTraits`     | Schema derived traits                        |
| `schemaEnums`           | Generated Rust schema enum values            |
| `version`               | Version of the embedded `cofd` crate        |
| `ready`                 | `true` once the WebAssembly module is ready  |
| `error`                 | The error that prevented loading, if any     |
| `instance`              | The generated UBRN exports module            |

The generated TypeScript files contain enum definitions and FFI signatures
derived from the Rust API. The bundled loader exposes the schema definitions
through `api.schemaEnums`.

A smoke message is written to the console when Foundry fires its `init` hook.

## Building

From the repository root:

```
pnpm --dir crates/cofd-foundry install --frozen-lockfile
crates/cofd-foundry/build-system.sh
```

This compiles the crate for `wasm32-unknown-unknown`, stages the system in
`crates/cofd-foundry/dist/cofd/`, and writes
`crates/cofd-foundry/dist/cofd/cofd.zip`.
