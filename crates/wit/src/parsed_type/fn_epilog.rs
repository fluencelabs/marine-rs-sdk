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
        Some(ParsedType::Boolean(_)) => Some("i32"),
        Some(ParsedType::I8(_)) => Some("i8"),
        Some(ParsedType::I16(_)) => Some("i16"),
        Some(ParsedType::I32(_)) => Some("i32"),
        Some(ParsedType::I64(_)) => Some("i64"),
        Some(ParsedType::U8(_)) => Some("u8"),
        Some(ParsedType::U16(_)) => Some("u16"),
        Some(ParsedType::U32(_)) => Some("u32"),
        Some(ParsedType::U64(_)) => Some("u64"),
        Some(ParsedType::F32(_)) => Some("f32"),
        Some(ParsedType::F64(_)) => Some("f64"),
        None
        | Some(ParsedType::Utf8Str(_))
        | Some(ParsedType::Utf8String(_))
        | Some(ParsedType::Vector(..))
        | Some(ParsedType::Record(..)) => None,
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
        Some(ParsedType::Record(..)) => {
            quote! {
                let result_ptr = result.__fce_generated_serialize();
                fluence::internal::set_result_ptr(result_ptr as _);
            }
        }
        Some(ParsedType::Utf8Str(_)) | Some(ParsedType::Utf8String(_)) => quote! {
            fluence::internal::set_result_ptr(result.as_ptr() as _);
            fluence::internal::set_result_size(result.len() as _);
            std::mem::forget(result);
        },
        Some(ParsedType::Vector(ty, _)) => {
            let generated_serializer_name = String::from("__fce_generated_vec_serializer");
            let generated_serializer_ident = new_ident!(generated_serializer_name);
            let vector_serializer =
                super::vector_utils::generate_vector_serializer(ty, &generated_serializer_name);

            quote! {
                #vector_serializer
                let result = #generated_serializer_ident(result);
                fluence::internal::set_result_ptr(result.0 as _);
                fluence::internal::set_result_size(result.1 as _);
                std::mem::forget(result);
            }
        }
        Some(_) => quote! {
            return result as _;
        },
    }
}
