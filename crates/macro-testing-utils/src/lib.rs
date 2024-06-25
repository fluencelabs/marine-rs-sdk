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

use std::io::Read;
use std::path::Path;

pub fn stream_from_file<P>(path: P) -> proc_macro2::TokenStream
where
    P: AsRef<Path>,
{
    let items = items_from_file(path);
    quote::quote! { #(#items)* }
}

pub fn items_from_file<P>(path: P) -> Vec<syn::Item>
where
    P: AsRef<Path>,
{
    let mut file = std::fs::File::open(path).expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let token_file = syn::parse_file(&src).expect("Unable to parse file");
    token_file.items
}

pub fn to_syn_item(token_stream: proc_macro2::TokenStream) -> Vec<syn::Item> {
    let file: syn::File = syn::parse2(token_stream).expect("token stream should be parsed");
    file.items
}
