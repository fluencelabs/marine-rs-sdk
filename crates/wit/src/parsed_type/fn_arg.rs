/*
 * copyright 2020 fluence labs limited
 *
 * licensed under the apache license, version 2.0 (the "license");
 * you may not use this file except in compliance with the license.
 * you may obtain a copy of the license at
 *
 *     http://www.apache.org/licenses/license-2.0
 *
 * unless required by applicable law or agreed to in writing, software
 * distributed under the license is distributed on an "as is" basis,
 * without warranties or conditions of any kind, either express or implied.
 * see the license for the specific language governing permissions and
 * limitations under the license.
 */

use super::ParsedType;
use crate::wasm_type::RustType;

/// This trait could be used to generate raw args needed to construct a export function.
pub(crate) trait FnArgGlueCodeGenerator {
    fn generate_arguments(&self) -> Vec<RustType>;
}

impl FnArgGlueCodeGenerator for (String, ParsedType) {
    fn generate_arguments(&self) -> Vec<RustType> {
        match self.1 {
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
