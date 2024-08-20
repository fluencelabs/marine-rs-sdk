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

use crate::parse_macro_input::ParseMacroInput;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Result;

pub fn marine(tokens: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2::<syn::Item>(tokens)?;
    // convert proc_macro2 token to internal AST type
    let marine_ast_item = item.parse_macro_input()?;

    // convert internal AST type to sequence of tokens
    let mut tokens = TokenStream::new();
    marine_ast_item.to_tokens(&mut tokens);

    Ok(tokens)
}
