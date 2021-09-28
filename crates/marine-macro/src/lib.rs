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

//! Defines the #[marine] macro that should be used with all export functions, extern blocks.
//! At now, It supports the following types that could be used as parameters in export or foreign
//! functions: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, String, Vec<u8>. Also struct
//! where all fields are public and have aforementioned types could be used as parameters. In this
//! case #[marine] should be also applied to this structs.
//!
//! # Examples
//!
//! This example shows how a function could be exported:
//! ```ignore
//! #[marine]
//! pub fn greeting(name: String) -> String {
//!     format!("Hi {}", name)
//! }
//! ```
//!
//! This more complex example shows how a function could be imported from another Wasm module
//! and how a struct could be passed:
//!
//! ```ignore
//! use marine_rs_sdk::MountedBinaryResult;
//!
//! #[marine]
//! pub fn read_ipfs_file(file_path: String) -> MountedBinaryResult {
//!     let hash = calculate_hash(file_path);
//!     ipfs(vec![hash])
//! }
//!
//! #[marine]
//! #[link(wasm_import_module = "ipfs_node")]
//! extern "C" {
//!     pub fn ipfs(file_hash: Vec<String>) -> MountedBinaryResult;
//! }
//!
//! ```

#![doc(html_root_url = "https://docs.rs/marine-macro/0.6.13")]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_unsafe,
    unreachable_patterns
)]
#![warn(rust_2018_idioms)]
#![recursion_limit = "1024"]

use marine_macro_impl::marine as marine_impl;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn marine(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // into converts proc_macro::TokenStream to proc_macro2::TokenStream
    match marine_impl(input.into()) {
        Ok(v) => v,
        // converts syn:error to proc_macro2::TokenStream
        Err(e) => e.to_compile_error(),
    }
    // converts proc_macro2::TokenStream to proc_macro::TokenStream
    .into()
}

// deprecated macro for backwards compatibility
#[deprecated(since = "0.6.2", note = "please use the #[marine] macro instead")]
#[proc_macro_attribute]
pub fn fce(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // into converts proc_macro::TokenStream to proc_macro2::TokenStream
    match marine_impl(input.into()) {
        Ok(v) => v,
        // converts syn:error to proc_macro2::TokenStream
        Err(e) => e.to_compile_error(),
    }
    // converts proc_macro2::TokenStream to proc_macro::TokenStream
    .into()
}
