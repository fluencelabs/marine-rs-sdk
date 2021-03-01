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

use proc_macro2::TokenStream;

/// Raw Wasm types according to the spec except i128.
pub enum RustType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

impl quote::ToTokens for RustType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let call_site = proc_macro2::Span::call_site();
        match self {
            RustType::U8 => syn::Ident::new("u8", call_site).to_tokens(tokens),
            RustType::U16 => syn::Ident::new("u16", call_site).to_tokens(tokens),
            RustType::U32 => syn::Ident::new("u32", call_site).to_tokens(tokens),
            RustType::U64 => syn::Ident::new("u64", call_site).to_tokens(tokens),
            RustType::I8 => syn::Ident::new("i8", call_site).to_tokens(tokens),
            RustType::I16 => syn::Ident::new("i16", call_site).to_tokens(tokens),
            RustType::I32 => syn::Ident::new("i32", call_site).to_tokens(tokens),
            RustType::I64 => syn::Ident::new("i64", call_site).to_tokens(tokens),
            RustType::F32 => syn::Ident::new("f32", call_site).to_tokens(tokens),
            RustType::F64 => syn::Ident::new("f64", call_site).to_tokens(tokens),
        }
    }
}
