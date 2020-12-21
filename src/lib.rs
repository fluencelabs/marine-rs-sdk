/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Rust backend SDK for applications on the Fluence network. This crate defines the procedure macro
//! `#[fce]` that could be applied to a function, structure or extern block.
//!
//! Structures with `#[fce]` (hereinafter they'll be called records) could be used then in function
//! arguments and values. All fields of a record should be public and have one of the
//! following primitive Rust types
//! (`bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, String, Vec<u8>`).
//! ```rust
//! use fluence::fce;
//!
//! #[fce]
//! struct T {
//!     pub field_1: i32,
//!     pub field_2: Vec<u8>,
//! }
//! ```
//!
//! Functions with `#[fce]` will be exported from this module:
//!
//! ```rust
//! use fluence::fce;
//!
//! #[fce]
//! pub fn get(url: String) {
//!     // ...
//! }
//! ```
//! At now, such functions could have arguments with primitive Rust types and record and only one
//! return argument with such type could be used.
//!
//! Finally, to import other wasm modules to your project use similar code:
//! ```rust
//! use fluence::fce;
//!
//! #[fce]
//! #[link(wasm_import_module = "wasm_curl.wasm")]
//! extern "C" {
//!     #[link_name = "get"]
//!     pub fn curl_get(url: String) -> String;
//! }
//! ```
#![doc(html_root_url = "https://docs.rs/fluence/0.2.11")]
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

pub use fluence_sdk_macro::fce;

pub use fluence_sdk_main::CallParameters;
pub use fluence_sdk_main::SecurityTetraplet;
#[cfg(target_arch = "wasm32")]
pub use fluence_sdk_main::get_call_parameters;

#[cfg(feature = "logger")]
pub use fluence_sdk_main::WasmLoggerBuilder;
#[cfg(feature = "logger")]
pub use fluence_sdk_main::TargetMap;

/// These API functions are intended for internal usage in generated code.
/// Normally, you shouldn't use them.
pub mod internal {
    pub use fluence_sdk_main::get_result_ptr;
    pub use fluence_sdk_main::get_result_size;
    pub use fluence_sdk_main::set_result_ptr;
    pub use fluence_sdk_main::set_result_size;
}
