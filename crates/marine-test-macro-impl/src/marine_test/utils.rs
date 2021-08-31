/*
 * Copyright 2021 Fluence Labs Limited
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

use crate::TResult;
use marine_it_parser::it_interface::IRecordTypes;
use marine_it_parser::it_interface::it::IType;

use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn new_ident(ident_str: &str) -> TResult<syn::Ident> {
    let ident_str = ident_str.replace('-', "_");
    syn::parse_str::<syn::Ident>(&ident_str).map_err(Into::into)
}

pub(super) fn itype_to_tokens(itype: &IType, records: &IRecordTypes) -> TResult<TokenStream> {
    let token_stream = match itype {
        IType::Record(record_id) => {
            let record = records
                .get(record_id)
                .ok_or_else(|| crate::errors::CorruptedITSection::AbsentRecord(*record_id))?;
            let record_name = new_ident(&record.name)?;
            let token_stream = quote! { #record_name };
            token_stream
        }
        IType::Array(ty) => {
            let inner_ty_token_stream = itype_to_tokens(ty, records)?;
            let token_stream = quote! { Vec<#inner_ty_token_stream> };
            token_stream
        }
        IType::String => quote! { String },
        IType::ByteArray => quote! { Vec<u8> },
        IType::Boolean => quote! { bool },
        IType::S8 => quote! { i8 },
        IType::S16 => quote! { i16 },
        IType::S32 => quote! { i32 },
        IType::S64 => quote! { i64 },
        IType::U8 => quote! { u8 },
        IType::U16 => quote! { u16 },
        IType::U32 => quote! { u32 },
        IType::U64 => quote! { u64 },
        IType::I32 => quote! { i32 },
        IType::I64 => quote! { i64 },
        IType::F32 => quote! { f32 },
        IType::F64 => quote! { f64 },
    };

    Ok(token_stream)
}
