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

#![doc(html_root_url = "https://docs.rs/fluence-test-macro/0.1.7")]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]
#![feature(proc_macro_span)]
#![warn(rust_2018_idioms)]
#![recursion_limit = "1024"]

use marine_test_macro_impl::marine_test_impl;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::spanned::Spanned;

/// This macro allows user to write tests for services in the following form:
///```rust
/// #[marine_test(config = "/path/to/Config.toml", modules_dir = "path/to/service/modules")]
/// fn test() {
///     let service_result = greeting.greeting("John".to_string());
///     assert_eq!(&service_result, "Hi, name!");
/// }
///```
#[proc_macro_error]
#[proc_macro_attribute]
pub fn marine_test(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: proc_macro2::TokenStream = attrs.into();
    let attrs_span = attrs.span();
    // here it obtains a path to the current file where macro is applied
    let mut file_path = proc_macro::Span::call_site().source_file().path();
    let _ = file_path.pop();

    match marine_test_impl(attrs, input.into(), file_path) {
        Ok(stream) => stream.into(),
        Err(e) => proc_macro_error::abort!(attrs_span, format!("{}", e)),
    }
}

// deprecated macro for backwards compatibility
#[deprecated(since = "0.6.2", note = "please use the #[marine] macro instead")]
#[proc_macro_error]
#[proc_macro_attribute]
pub fn fce_test(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: proc_macro2::TokenStream = attrs.into();
    let attrs_span = attrs.span();
    // here it obtains a path to the current file where macro is applied
    let mut file_path = proc_macro::Span::call_site().source_file().path();
    let _ = file_path.pop();

    match marine_test_impl(attrs, input.into(), file_path) {
        Ok(stream) => stream.into(),
        Err(e) => proc_macro_error::abort!(attrs_span, format!("{}", e)),
    }
}
