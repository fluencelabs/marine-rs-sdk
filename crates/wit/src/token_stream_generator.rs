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

pub const GENERATED_WRAPPER_FUNC_PREFIX: &str = "__fce_generated_wrapper_func_";
pub const GENERATED_SECTION_PREFIX: &str = "__fce_generated_section__";
pub const GENERATED_GLOBAL_PREFIX: &str = "__fce_generated_static_global_";

impl quote::ToTokens for FCEAst {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FCEAst::Function(ast_function) => ast_function.to_tokens(tokens),
            FCEAst::ExternMod(ast_extern) => ast_extern.to_tokens(tokens),
            FCEAst::Record(ast_record) => ast_record.to_tokens(tokens),
        }
    }
}
