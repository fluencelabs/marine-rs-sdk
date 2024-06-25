/*
 * Fluence Marine Rust SDK
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! Rust backend SDK for applications on the Fluence network. This crate defines the procedure macro
//! `#[marine]` that could be applied to a function, structure or extern block.
//!
//! Structures with `#[marine]` (hereinafter they'll be called records) could be used then in function
//! arguments and values. All fields of a record should be public and have one of the
//! following primitive Rust types
//! (`bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, String, Vec<u8>`).
//! ```rust
//! use marine_rs_sdk::marine;
//!
//! #[marine]
//! struct T {
//!     pub field_1: i32,
//!     pub field_2: Vec<u8>,
//! }
//! ```
//!
//! Functions with `#[marine]` will be exported from this module:
//!
//! ```rust
//! use marine_rs_sdk::marine;
//!
//! #[marine]
//! pub fn get(url: String) {
//!     // ...
//! }
//! ```
//! At now, such functions could have arguments with primitive Rust types and record and only one
//! return argument with such type could be used.
//!
//! Finally, to import other wasm modules to your project use similar code:
//! ```rust
//! use marine_rs_sdk::marine;
//!
//! #[marine]
//! #[link(wasm_import_module = "wasm_curl.wasm")]
//! extern "C" {
//!     #[link_name = "get"]
//!     pub fn curl_get(url: String) -> String;
//! }
//! ```
#![doc(html_root_url = "https://docs.rs/sdk/0.14.0")] // x-release-please-version
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]
#![warn(rust_2018_idioms)]

mod mounted_binary;

#[allow(unused_extern_crates)]
// sdk is used inside CallParameters and MountedBinaryResult glue code
extern crate self as marine_rs_sdk;

pub use marine_macro::marine;
pub use marine_macro::fce;

pub use marine_call_parameters::CallParameters;
pub use marine_call_parameters::ParticleParameters;
pub use marine_call_parameters::SecurityTetraplet;
pub use marine_call_parameters::get_call_parameters;

#[cfg(feature = "logger")]
pub use marine_rs_sdk_main::WasmLoggerBuilder;
#[cfg(feature = "logger")]
pub use marine_rs_sdk_main::TargetMap;

pub use mounted_binary::MountedBinaryResult;
pub use mounted_binary::MountedBinaryStringResult;
pub use mounted_binary::SUCCESS_CODE as BINARY_SUCCESS_CODE;

pub use marine_rs_sdk_main::module_manifest;

/// These API functions are intended for internal usage in generated code.
/// Normally, you shouldn't use them.
#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
#[doc(hidden)]
pub mod internal {
    pub use marine_rs_sdk_main::get_result_ptr;
    pub use marine_rs_sdk_main::get_result_size;
    pub use marine_rs_sdk_main::set_result_ptr;
    pub use marine_rs_sdk_main::set_result_size;
    pub use marine_rs_sdk_main::add_object_to_release;
    pub use marine_timestamp_macro::build_timestamp;
}

#[cfg(not(feature = "no-explicit-ctors-call"))]
#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
extern "C" {
    // For internal use. Not an API function.
    fn __wasm_call_ctors();
}

// Adds an explicit __wasm_call_ctors call to tell LLVM not to
// wrap every export in __wasm_call_ctors/__wasm_call_dtors calls.
// The most referenced issue about it is https://github.com/WebAssembly/WASI/issues/471
// For internal use. Not an API function.
#[cfg(not(feature = "no-explicit-ctors-call"))]
#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
#[doc(hidden)]
#[no_mangle]
pub fn _initialize() {
    unsafe {
        __wasm_call_ctors();
    }
}
