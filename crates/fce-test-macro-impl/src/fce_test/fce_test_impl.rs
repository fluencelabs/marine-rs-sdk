/*
 * Copyright 2021 Fluence Labs Limited
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

use crate::attributes::FCETestAttributes;
use crate::TResult;
use crate::fce_test::glue_code_generator::generate_test_glue_code;

use proc_macro2::TokenStream;
use darling::FromMeta;
use syn::parse::Parser;
use std::path::PathBuf;

pub fn fce_test_impl(
    attrs: TokenStream,
    input: TokenStream,
    full_path: PathBuf,
) -> TResult<TokenStream> {
    // from https://github.com/dtolnay/syn/issues/788
    let parser = syn::punctuated::Punctuated::<syn::NestedMeta, syn::Token![,]>::parse_terminated;
    let attrs = parser.parse2(attrs)?;
    let attrs: Vec<syn::NestedMeta> = attrs.into_iter().collect();
    let attrs = FCETestAttributes::from_list(&attrs)?;

    let func_item = syn::parse2::<syn::ItemFn>(input)?;

    generate_test_glue_code(func_item, attrs, full_path)
}
