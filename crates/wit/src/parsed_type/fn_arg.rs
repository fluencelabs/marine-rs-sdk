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
use crate::wasm_type::WasmType;

/// This trait could be used to generate raw args needed to construct a export function.
pub(crate) trait FnArgGlueCodeGenerator {
    fn generate_arguments(&self) -> Vec<WasmType>;
}

impl FnArgGlueCodeGenerator for ParsedType {
    fn generate_arguments(&self) -> Vec<WasmType> {
        match self {
            ParsedType::Boolean
            | ParsedType::I8
            | ParsedType::I16
            | ParsedType::I32
            | ParsedType::U8
            | ParsedType::U16
            | ParsedType::U32 => vec![WasmType::I32],
            ParsedType::I64 | ParsedType::U64 => vec![WasmType::I64],
            ParsedType::F32 => vec![WasmType::F32],
            ParsedType::F64 => vec![WasmType::F64],
            ParsedType::Utf8String | ParsedType::ByteVector | ParsedType::Record(_) => {
                vec![WasmType::I32, WasmType::I32]
            }
        }
    }
}
