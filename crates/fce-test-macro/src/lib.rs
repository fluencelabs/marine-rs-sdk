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

#![doc(html_root_url = "https://docs.rs/fluence-sdk-macro/0.5.0")]
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

use fluence_sdk_test_macro_impl::fce_test_impl;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::spanned::Spanned;

/// This macro allows user to write tests for services in the following form:
///```ignore
/// #[fce_test(config = "/path/to/Config.toml", modules_dir = "path/to/service/modules")]
/// fn test() {
///     let service_result = greeting.greeting("John".to_string());
///     assert_eq!(&service_result, "Hi, name!");
/// }
///```
#[proc_macro_error]
#[proc_macro_attribute]
pub fn fce_test(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: proc_macro2::TokenStream = attrs.into();
    let attrs_span = attrs.span();

    match fce_test_impl(attrs, input.into()) {
        Ok(stream) => stream.into(),
        Err(e) => proc_macro_error::abort!(attrs_span, format!("{}", e)),
    }
}
