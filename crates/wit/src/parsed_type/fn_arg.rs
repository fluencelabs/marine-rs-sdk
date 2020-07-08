use super::ParsedType;
use crate::wasm_type::WasmType;

/// This trait could be used to generate raw args needed to construct a export function.
pub(crate) trait FnArgGlueCodeGenerator {
    fn generate_arguments(&self) -> Vec<WasmType>;
}

impl FnArgGlueCodeGenerator for ParsedType {
    fn generate_arguments(&self) -> Vec<WasmType> {
        // TODO: investigate possible issues in conversion between signed and unsigned types
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
