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

#![doc(html_root_url = "https://docs.rs/marine-macro-impl/0.14.0")] // x-release-please-version
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]
#![recursion_limit = "1024"]
#![warn(rust_2018_idioms)]

/// This crate contains functions and types to support work with WebAssembly interface-types
/// in Fluence.

mod ast_types;
mod export_ast_types;
mod marine_macro_impl;
mod parsed_type;
mod parse_macro_input;
mod token_stream_generator;
mod utils;
mod wasm_type;

pub use export_ast_types::*;
pub use crate::marine_macro_impl::marine;
pub use parsed_type::ParsedType;
pub use parsed_type::PassingStyle;
pub use token_stream_generator::GENERATED_WRAPPER_FUNC_PREFIX;
pub use token_stream_generator::GENERATED_SECTION_PREFIX;
pub use token_stream_generator::GENERATED_GLOBAL_PREFIX;
pub use wasm_type::RustType;

pub const GENERATED_SECTION_PREFIX_FCE: &str = "__fce_generated_section__";
pub const MARINE_HOST_API_NAMESPACE_PREFIX: &str = "__marine_host_api_v";
pub const MARINE_HOST_API_VERSION: u32 = 1;
