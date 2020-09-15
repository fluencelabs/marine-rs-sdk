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

use super::ParsedType;
use crate::wasm_type::RustType;

/// This trait could be used to generate raw args needed to construct a export function.
pub(crate) trait FnArgGlueCodeGenerator {
    fn generate_arguments(&self) -> Vec<RustType>;
}

impl FnArgGlueCodeGenerator for (String, ParsedType) {
    fn generate_arguments(&self) -> Vec<RustType> {
        match self.1 {
            ParsedType::Boolean => vec![RustType::I32],
            ParsedType::I8 => vec![RustType::I8],
            ParsedType::I16 => vec![RustType::I16],
            ParsedType::I32 => vec![RustType::I32],
            ParsedType::I64 => vec![RustType::I64],
            ParsedType::U8 => vec![RustType::U8],
            ParsedType::U16 => vec![RustType::U16],
            ParsedType::U32 => vec![RustType::U32],
            ParsedType::U64 => vec![RustType::U64],
            ParsedType::Record(_) => vec![RustType::U32],
            ParsedType::F32 => vec![RustType::F32],
            ParsedType::F64 => vec![RustType::F64],
            ParsedType::Utf8String | ParsedType::ByteVector => vec![RustType::U32, RustType::U32],
        }
    }
}
