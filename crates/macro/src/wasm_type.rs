use proc_macro2::TokenStream;
use quote::TokenStreamExt;

pub(crate) enum WasmType {
    I32,
    I64,
    F32,
    F64,
}

impl quote::ToTokens for WasmType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.clone());
    }
}
