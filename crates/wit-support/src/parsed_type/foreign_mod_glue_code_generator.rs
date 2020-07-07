/*
 * Copyright 2018 Fluence Labs Limited
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

use quote::quote;

pub(crate) trait ForeignModeGlueCodeGenerator {
    fn generate_wrapper_sign_expression(&self) -> proc_macro2::TokenStream;

    fn generate_input_type(&self) -> proc_macro2::TokenStream;

    fn generate_raw_args(&self, arg_start_id: usize) -> proc_macro2::TokenStream;

    fn generate_wrapper_epilog(&self) -> proc_macro2::TokenStream;
}

impl ForeignModeGlueCodeGenerator for ParsedType {
    fn generate_wrapper_sign_expression(&self) -> proc_macro2::TokenStream {
        match self {
            ParsedType::Empty => quote! {},
            ty @ _ => {
                let ty = syn::Ident::new(&ty.to_text_type(), proc_macro2::Span::call_site());
                quote! { -> #ty }
            }
        }
    }

    fn generate_input_type(&self) -> proc_macro2::TokenStream {
        use quote::ToTokens;

        match self {
            ParsedType::Empty => quote! {},
            ty @ _ => syn::Ident::new(&ty.to_text_type(), proc_macro2::Span::call_site())
                .to_token_stream(),
        }
    }

    fn generate_raw_args(&self, arg_start_id: usize) -> proc_macro2::TokenStream {
        let arg = syn::Ident::new(
            &format!("arg_{}", arg_start_id),
            proc_macro2::Span::call_site(),
        );

        match self {
            ParsedType::Utf8String => quote! { #arg.as_ptr() as _, #arg.len() as _ },
            ParsedType::ByteVector => quote! { #arg.as_ptr() as _, #arg.len() as _ },
            _ => quote! { arg },
        }
    }

    fn generate_wrapper_epilog(&self) -> proc_macro2::TokenStream {
        match self {
            ParsedType::Empty => quote! {},
            ParsedType::I8 => quote! {
                return result as _;
            },
            ParsedType::I16 => quote! {
                return result as _;
            },
            ParsedType::I32 => quote! {
                return result as _;
            },
            ParsedType::I64 => quote! {
                return result as _;
            },
            ParsedType::U8 => quote! {
                return result as _;
            },
            ParsedType::U16 => quote! {
                return result as _;
            },
            ParsedType::U32 => quote! {
                return result as _;
            },
            ParsedType::U64 => quote! {
                return result as _;
            },
            ParsedType::F32 => quote! {
                return result as _;
            },
            ParsedType::F64 => quote! {
                return result as _;
            },
            ParsedType::Boolean => quote! {
                return result as _;
            },
            ParsedType::Utf8String => quote! {
                String::from_raw_parts(
                    fluence::get_result_ptr() as _,
                    fluence::get_result_size() as _,
                    fluence::get_result_size() as _
                )
            },
            ParsedType::ByteVector => quote! {
                Vec::from_raw_parts(
                    fluence::get_result_ptr() as _,
                    fluence::get_result_size() as _,
                    fluence::get_result_size() as _
                )
            },
            ParsedType::Record(_) => quote! {
                fluence::set_result_ptr(result.as_ptr() as _);
                fluence::set_result_size(result.len() as _);
                std::mem::forget(result);
            },
        }
    }
}
