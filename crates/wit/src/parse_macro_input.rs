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

mod item_fn;
mod item_foreign_mod;
mod item_record;
mod utils;

use crate::ast_types::FCEAst;

pub(crate) trait ParseMacroInput {
    fn parse_macro_input(self) -> syn::Result<FCEAst>;
}

impl ParseMacroInput for syn::Item {
    fn parse_macro_input(self) -> syn::Result<FCEAst> {
        use syn::spanned::Spanned;

        match self {
            syn::Item::Fn(function) => function.parse_macro_input(),
            syn::Item::ForeignMod(extern_mod) => extern_mod.parse_macro_input(),
            syn::Item::Struct(item_struct) => item_struct.parse_macro_input(),
            _ => Err(syn::Error::new(
                self.span(),
                "At now, #[fce] could be applied only to a function, extern block or struct",
            )),
        }
    }
}
