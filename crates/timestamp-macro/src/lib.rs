/*
 * Marine Rust SDK
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

use proc_macro::TokenStream;

#[proc_macro]
pub fn build_timestamp(_: TokenStream) -> TokenStream {
    let current_utc_date = chrono::Utc::now();
    let current_utc_date = current_utc_date.to_rfc3339();

    let glue_code = quote::quote! { const __M_SDK_BUILD_TIME: &str = #current_utc_date; };
    glue_code.into()
}
