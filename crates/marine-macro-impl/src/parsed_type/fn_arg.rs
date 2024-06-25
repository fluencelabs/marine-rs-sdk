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

use super::ParsedType;
use crate::ast_types::AstFnArgument;
use crate::wasm_type::RustType;

/// This trait could be used to generate raw args needed to construct a export function.
pub(crate) trait FnArgGlueCodeGenerator {
    fn generate_arguments(&self) -> Vec<RustType>;
}

impl FnArgGlueCodeGenerator for AstFnArgument {
    fn generate_arguments(&self) -> Vec<RustType> {
        match self.ty {
            ParsedType::Boolean(_) => vec![RustType::I32],
            ParsedType::I8(_) => vec![RustType::I8],
            ParsedType::I16(_) => vec![RustType::I16],
            ParsedType::I32(_) => vec![RustType::I32],
            ParsedType::I64(_) => vec![RustType::I64],
            ParsedType::U8(_) => vec![RustType::U8],
            ParsedType::U16(_) => vec![RustType::U16],
            ParsedType::U32(_) => vec![RustType::U32],
            ParsedType::U64(_) => vec![RustType::U64],
            ParsedType::Record(..) => vec![RustType::U32],
            ParsedType::F32(_) => vec![RustType::F32],
            ParsedType::F64(_) => vec![RustType::F64],
            ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) | ParsedType::Vector(..) => {
                vec![RustType::U32, RustType::U32]
            }
        }
    }
}
