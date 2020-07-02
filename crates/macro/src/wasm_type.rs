use proc_macro2::TokenStream;

pub(crate) enum WasmType {
    I32,
    I64,
    F32,
    F64,
}

impl quote::ToTokens for WasmType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            WasmType::I32 => "i32".to_tokens(tokens),
            WasmType::I64 => "i64".to_tokens(tokens),
            WasmType::F32 => "f32".to_tokens(tokens),
            WasmType::F64 => "f64".to_tokens(tokens),
        }
    }
}
