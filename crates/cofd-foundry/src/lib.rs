//! WebAssembly FFI between the `cofd` rules engine and Foundry VTT.
//!
//! The crate is compiled as a `cdylib` for `wasm32-unknown-unknown`. The
//! `wasm-bindgen` CLI generates the JavaScript loader and bindings consumed by
//! the Foundry system.

mod ffi;

use wasm_bindgen::prelude::*;

/// Version of the embedded rules engine.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[wasm_bindgen]
pub fn version() -> String {
	VERSION.to_owned()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn version_is_the_package_version() {
		assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
		assert!(!VERSION.is_empty());
	}
}
