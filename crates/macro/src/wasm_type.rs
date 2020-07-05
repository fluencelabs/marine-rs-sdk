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
            WasmType::I32 => {
                syn::Ident::new("i32", proc_macro2::Span::call_site()).to_tokens(tokens)
            }
            WasmType::I64 => syn::parse_str::<syn::Ident>("i64")
                .unwrap()
                .to_tokens(tokens),
            WasmType::F32 => syn::parse_str::<syn::Ident>("f32")
                .unwrap()
                .to_tokens(tokens),
            WasmType::F64 => syn::parse_str::<syn::Ident>("f64")
                .unwrap()
                .to_tokens(tokens),
        }
    }
}
