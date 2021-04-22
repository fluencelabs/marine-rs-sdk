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

mod der;
mod ser;

use ser::*;
use der::*;
use super::ParsedType;

use quote::quote;

pub(crate) fn generate_vector_ser(
    value_ty: &ParsedType,
    arg_name: &str,
) -> proc_macro2::TokenStream {
    let values_ser = match value_ty {
        ParsedType::Boolean(_) => {
            quote! {
                let converted_bool_vector: Vec<u8> = arg.into_iter().map(|v| v as u8).collect::<_>();
                fluence::internal::add_object_to_release(Box::new(converted_bool_vector));
                (converted_bool_vector.as_ptr() as _, converted_bool_vector.len() as _)
            }
        },
        ParsedType::I8(_)
        | ParsedType::U8(_)
        | ParsedType::I16(_)
        | ParsedType::U16(_)
        | ParsedType::I32(_)
        | ParsedType::U32(_)
        | ParsedType::I64(_)
        | ParsedType::U64(_)
        | ParsedType::F32(_)
        | ParsedType::F64(_) => {
            quote! {
                (arg.as_ptr() as _, arg.len() as _)
            }
        }
        ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => string_ser(),
        ParsedType::Vector(ty, _) => vector_ser(arg_name, ty),
        ParsedType::Record(..) => record_ser(),
    };

    let arg = crate::new_ident!(arg_name);

    quote! {
        unsafe fn #arg(arg: &Vec<#value_ty>) -> (u32, u32) {
            #values_ser
        }
    }
}

pub(crate) fn generate_vector_der(
    value_ty: &ParsedType,
    arg_name: &str,
) -> proc_macro2::TokenStream {
    let arg = crate::new_ident!(arg_name);

    let values_deserializer = match value_ty {
        ParsedType::Boolean(_) => {
            quote! {
                let arg: Vec<u8> = Vec::from_raw_parts(offset as _, size as _, size as _);
                arg.into_iter().map(|v| v != 0).collect::<Vec<bool>>()
            }
        }
        ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => string_der(),
        ParsedType::Vector(ty, _) => vector_der(arg_name, ty),
        ParsedType::Record(record_name, _) => record_der(record_name),
        _ => {
            quote! {
                Vec::from_raw_parts(offset as _, size as _, size as _)
            }
        }
    };

    quote! {
        unsafe fn #arg(offset: u32, size: u32) -> Vec<#value_ty> {
            #values_deserializer
        }
    }
}
