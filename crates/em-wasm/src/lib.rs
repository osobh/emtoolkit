#![allow(clippy::too_many_arguments)]
//! WebAssembly bindings for the EM Toolkit.
//!
//! Each module exposes computation functions that accept and return
//! JsValue (via serde_wasm_bindgen) for direct use from JavaScript/TypeScript.
//!
//! Convention: functions take JSON-serializable params and return JsValue results.

use wasm_bindgen::prelude::*;

pub mod bind_core;
pub mod bind_waves;
pub mod bind_transmission;
pub mod bind_vectors;
pub mod bind_electrostatics;
pub mod bind_magnetostatics;
pub mod bind_timevarying;
pub mod bind_propagation;
pub mod bind_antennas;

/// Initialize the WASM module (call once from JS).
#[wasm_bindgen]
pub fn init() {
    // Panic hook can be added when console_error_panic_hook dep is included
}

/// Get the toolkit version.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
