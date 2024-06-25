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
