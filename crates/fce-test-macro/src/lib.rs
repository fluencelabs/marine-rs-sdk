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

#![doc(html_root_url = "https://docs.rs/fluence-sdk-macro/0.4.2")]
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
#![recursion_limit = "1024"]

mod attributes;
mod fce_test;

use fce_test::fce_test_impl;
use proc_macro::TokenStream;

/// This macro allows user to write tests for services in the following form:
///```ignore
/// #[fce_test(config = "/path/to/Config.toml")]
/// fn test() {
///     let service_result = fce.call("greeting", "name");
///     assert_eq!(&service_result, "Hi, name!");
/// }
///```
///
/// This function is desugrated in the following way:
///```ignore
/// #[test]
/// fn test() {
///     let fce = fluence_faas::FluenceFaaS::with_raw_config("/path/to/Config.toml")
///         .unwrap_or_else(|e| panic!("test instance can't be instantiated: {}", e));
///     let service_result = fce.call("greeting", "name");
///     assert_eq!(&service_result, "Hi, name!");
/// }
///```
#[proc_macro_attribute]
pub fn fce_test(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let func_input = syn::parse_macro_input!(input as syn::ItemFn);
    fce_test_impl(attrs.into(), func_input)
}
