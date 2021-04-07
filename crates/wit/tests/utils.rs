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

use fluence_sdk_wit::fce;

use pretty_assertions::assert_eq;

use std::io::Read;
use std::path::Path;

pub fn test_fce_token_streams<FP, EP>(fce_path: FP, expanded_path: EP) -> bool
where
    FP: AsRef<Path>,
    EP: AsRef<Path>,
{
    let fce_item = stream_from_file(fce_path);
    let test_token_stream = quote::quote! { #fce_item };
    let fce_token_streams = fce(test_token_stream)
        .unwrap_or_else(|e| panic!("failed to apply the fce macro due {}", e));

    let expanded_item = items_from_file(expanded_path);
    let fce_item = to_syn_item(fce_token_streams);

    assert_eq!(expanded_item, fce_item);

    fce_item == expanded_item
}

fn stream_from_file<P>(path: P) -> proc_macro2::TokenStream
where
    P: AsRef<Path>,
{
    let items = items_from_file(path);
    quote::quote! { #(#items)* }
}

fn items_from_file<P>(path: P) -> Vec<syn::Item>
where
    P: AsRef<Path>,
{
    let mut file = std::fs::File::open(path).expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let token_file = syn::parse_file(&src).expect("Unable to parse file");
    token_file.items
}

fn to_syn_item(token_stream: proc_macro2::TokenStream) -> Vec<syn::Item> {
    let file: syn::File = syn::parse2(token_stream).expect("token stream should be parsed");
    file.items
}
