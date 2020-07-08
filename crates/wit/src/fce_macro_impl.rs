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

use crate::parse_macro_input::ParseMacroInput;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Result;

pub fn fce(tokens: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2::<syn::Item>(tokens)?;
    // convert proc_macro2 token to internal AST type
    let fce_ast_item = item.parse_macro_input()?;

    // convert internal AST type to sequence of tokens
    let mut tokens = TokenStream::new();
    fce_ast_item.to_tokens(&mut tokens);

    Ok(tokens)
}
