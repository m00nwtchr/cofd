//! WebAssembly FFI between the `cofd` rules engine and Foundry VTT.
//!
//! The crate is compiled as a `cdylib` for `wasm32-unknown-unknown`. UniFFI
//! metadata is used to generate the JavaScript/TypeScript bindings consumed by
//! the Foundry system.

#[cfg(target_arch = "wasm32")]
extern crate uniffi_runtime_wasm as _;

#[cfg(feature = "uniffi")]
mod ffi;

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

/// Version of the embedded rules engine.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "uniffi")]
#[uniffi::export]
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
