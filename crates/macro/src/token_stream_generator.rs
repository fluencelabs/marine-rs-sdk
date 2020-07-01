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

mod fn_generator;
mod foreign_mod_generator;
mod record_generator;

use crate::fce_ast_types::FCEAst;

use proc_macro2::TokenStream;

pub(crate) trait TokenStreamGenerator {
    fn generate_token_stream(self) -> syn::Result<TokenStream>;
}

impl TokenStreamGenerator for FCEAst {
    fn generate_token_stream(self) -> syn::Result<TokenStream> {
        match self {
            FCEAst::Function(ast_function) => ast_function.generate_token_stream(),
            FCEAst::ExternMod(ast_extern) => ast_extern.generate_token_stream(),
            FCEAst::Record(ast_record) => ast_record.generate_token_stream(),
        }
    }
}
