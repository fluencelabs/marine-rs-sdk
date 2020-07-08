use proc_macro2::TokenStream;

/// Raw Wasm types according to the spec except i128.
pub(crate) enum WasmType {
    I32,
    I64,
    F32,
    F64,
}

impl quote::ToTokens for WasmType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let call_site = proc_macro2::Span::call_site();
        match self {
            WasmType::I32 => syn::Ident::new("i32", call_site).to_tokens(tokens),
            WasmType::I64 => syn::Ident::new("i64", call_site).to_tokens(tokens),
            WasmType::F32 => syn::Ident::new("f32", call_site).to_tokens(tokens),
            WasmType::F64 => syn::Ident::new("f64", call_site).to_tokens(tokens),
        }
    }
}
