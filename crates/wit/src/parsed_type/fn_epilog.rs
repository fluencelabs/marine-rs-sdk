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
use crate::new_ident;
use crate::token_stream_generator::GENERATED_RECORD_SERIALIZER_PREFIX;

use quote::quote;

/// Describes various parts of a function epilog.
pub(crate) struct FnEpilogDescriptor {
    pub(crate) fn_return_type: proc_macro2::TokenStream,
    pub(crate) return_expression: proc_macro2::TokenStream,
    pub(crate) epilog: proc_macro2::TokenStream,
}

/// This trait could be used to generate various parts needed to construct epilog of an export
/// function. They are marked with # in the following example:
/// ```
/// quote! {
///     pub unsafe fn foo(...) #fn_return_type {
///         ...
///         #return_expression original_foo(...);
///         #epilog
///     }
/// }
/// ```
pub(crate) trait FnEpilogGlueCodeGenerator {
    fn generate_fn_epilog(&self) -> FnEpilogDescriptor;
}

impl FnEpilogGlueCodeGenerator for Option<ParsedType> {
    fn generate_fn_epilog(&self) -> FnEpilogDescriptor {
        FnEpilogDescriptor {
            fn_return_type: generate_fn_return_type(self),
            return_expression: generate_return_expression(self),
            epilog: generate_epilog(self),
        }
    }
}

fn generate_fn_return_type(ty: &Option<ParsedType>) -> proc_macro2::TokenStream {
    let ty = match ty {
        Some(ParsedType::Boolean)
        | Some(ParsedType::I8)
        | Some(ParsedType::I16)
        | Some(ParsedType::I32)
        | Some(ParsedType::U8)
        | Some(ParsedType::U16)
        | Some(ParsedType::U32) => Some("i32"),

        Some(ParsedType::I64) | Some(ParsedType::U64) => Some("i64"),

        Some(ParsedType::F32) => Some("f32"),
        Some(ParsedType::F64) => Some("f64"),

        None
        | Some(ParsedType::Utf8String)
        | Some(ParsedType::ByteVector)
        | Some(ParsedType::Record(_)) => None,
    };

    match ty {
        Some(ty) => {
            let ty = new_ident!(ty);
            quote! { -> #ty}
        }
        None => quote! {},
    }
}

fn generate_return_expression(ty: &Option<ParsedType>) -> proc_macro2::TokenStream {
    match ty {
        None => quote! {},
        _ => quote! {
            let result =
        },
    }
}

fn generate_epilog(ty: &Option<ParsedType>) -> proc_macro2::TokenStream {
    match ty {
        None => quote!(),
        Some(ty) if !ty.is_complex_type() => quote! {
            return result as _;
        },
        Some(ParsedType::Record(record_name)) => {
            let record_serializer =
                crate::new_ident!(GENERATED_RECORD_SERIALIZER_PREFIX.to_string() + record_name);
            quote! {
                let result_ptr = crate::#record_serializer(result);
                fluence::internal::set_result_ptr(result_ptr as _);
            }
        }
        Some(ty) if ty.is_complex_type() => quote! {
            fluence::internal::set_result_ptr(result.as_ptr() as _);
            fluence::internal::set_result_size(result.len() as _);
            std::mem::forget(result);
        },
        _ => {
            panic!("perhaps new type's been added to ParsedType, and this match became incomplete")
        }
    }
}
