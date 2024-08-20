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

mod fn_generator;
mod foreign_mod_generator;
mod record_generator;

use crate::ast_types::MarineAst;

pub const GENERATED_WRAPPER_FUNC_PREFIX: &str = "__m_generated_wrapper_func_";
pub const GENERATED_SECTION_PREFIX: &str = "__m_generated_section__";
pub const GENERATED_GLOBAL_PREFIX: &str = "__m_generated_static_global_";

impl quote::ToTokens for MarineAst {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            MarineAst::Function(ast_function) => ast_function.to_tokens(tokens),
            MarineAst::ExternMod(ast_extern) => ast_extern.to_tokens(tokens),
            MarineAst::Record(ast_record) => ast_record.to_tokens(tokens),
        }
    }
}
