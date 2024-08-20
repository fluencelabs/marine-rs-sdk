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

mod item_fn;
mod item_foreign_mod;
mod item_record;
mod utils;

use crate::ast_types::MarineAst;

pub(crate) trait ParseMacroInput {
    fn parse_macro_input(self) -> syn::Result<MarineAst>;
}

impl ParseMacroInput for syn::Item {
    fn parse_macro_input(self) -> syn::Result<MarineAst> {
        use syn::spanned::Spanned;

        match self {
            syn::Item::Fn(function) => function.parse_macro_input(),
            syn::Item::ForeignMod(extern_mod) => extern_mod.parse_macro_input(),
            syn::Item::Struct(item_struct) => item_struct.parse_macro_input(),
            _ => Err(syn::Error::new(
                self.span(),
                "At now, #[marine] could be applied only to a function, extern block or struct",
            )),
        }
    }
}
