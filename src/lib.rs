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

//! Rust backend SDK for applications on the Fluence network. This crate is just a wrapper for two
//! other crates: `main` and `macro`. The `main` crate is used for all memory relative operations
//! and logging, while the `macro` crate contains the invocation macro to simplify entry point
//! functions.
//!
#![doc(html_root_url = "https://docs.rs/fluence/0.2.0")]
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

extern crate fluence_sdk_macro;
extern crate fluence_sdk_main;

pub use fluence_sdk_macro::fce;
pub use fluence_sdk_main::WasmLogger;

/// These API functions are intended for internal usage in generated code.
/// Normally, you shouldn't use them.
pub mod internal {
    pub use fluence_sdk_main::get_result_ptr;
    pub use fluence_sdk_main::get_result_size;
    pub use fluence_sdk_main::set_result_ptr;
    pub use fluence_sdk_main::set_result_size;
}
