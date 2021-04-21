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
use super::PassingStyle;

use quote::quote;

pub(crate) fn generate_vector_serializer(
    value_ty: &ParsedType,
    _vec_passing_style: PassingStyle,
    arg_name: &str,
) -> proc_macro2::TokenStream {
    let values_serializer = match value_ty {
        ParsedType::Boolean(_)
        | ParsedType::I8(_)
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
        ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => {
            quote! {
                let mut result: Vec<u32> = Vec::with_capacity(arg.len());

                for value in arg {
                    result.push(value.as_ptr() as _);
                    result.push(value.len() as _);
                }

                let result_ptr = result.as_ptr();
                let result_len = result.len();
                fluence::internal::add_object_to_release(Box::new(result));

                (result_ptr as _, result_len as _)
            }
        }
        ParsedType::Vector(ty, passing_style) => {
            let ser_name = format!("{}_{}", arg_name, ty);
            let ser_name = crate::utils::prepare_ident(ser_name);
            let ser_ident = crate::new_ident!(ser_name);

            let inner_vector_ser = generate_vector_serializer(&*ty, *passing_style, &ser_name);

            quote! {
                #inner_vector_ser

                let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
                for value in arg {
                    let (ptr, size) = #ser_ident(&value);
                    result.push(ptr as _);
                    result.push(size as _);
                }

                let result_ptr = result.as_ptr();
                let result_len = result.len();
                fluence::internal::add_object_to_release(Box::new(result));

                (result_ptr as _, result_len as _)
            }
        }

        ParsedType::Record(..) => {
            quote! {
                let mut result: Vec<u32> = Vec::with_capacity(arg.len());

                for value in arg {
                    result.push(value.__fce_generated_serialize() as _);
                }

                let result_ptr = result.as_ptr();
                let result_len = result.len();
                fluence::internal::add_object_to_release(Box::new(result));

                (result_ptr as _, result_len as _)
            }
        }
    };

    let arg = crate::new_ident!(arg_name);

    quote! {
        unsafe fn #arg(arg: &Vec<#value_ty>) -> (u32, u32) {
            #values_serializer
        }
    }
}

pub(crate) fn generate_vector_deserializer(
    value_ty: &ParsedType,
    arg_name: &str,
) -> proc_macro2::TokenStream {
    let arg = crate::new_ident!(arg_name);

    let values_deserializer = match value_ty {
        ParsedType::Boolean(_) => {
            quote! {
                let arg: Vec<u8> = Vec::from_raw_parts(offset as _, size as _, size as _);
                arg.into_iter().map(|v| v == 1).collect::<Vec<bool>>()
            }
        }
        ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => {
            quote! {
                let vec_passing_size = 2;
                let mut arg: Vec<u32> = Vec::from_raw_parts(offset as _, (vec_passing_size * size) as _, (vec_passing_size * size) as _);
                let mut arg = arg.into_iter();
                let mut result = Vec::with_capacity(arg.len() / 2);

                while let Some(offset) = arg.next() {
                    let size = arg.next().unwrap();
                    let value = String::from_raw_parts(offset as _, size as _, size as _);
                    result.push(value);
                }

                result
            }
        }
        ParsedType::Vector(ty, _) => {
            let deserializer_name = format!("{}_{}", arg_name, ty);
            let deserializer_name = crate::utils::prepare_ident(deserializer_name);
            let deserializer_ident = crate::new_ident!(deserializer_name);

            let inner_vector_deserializer = generate_vector_deserializer(&*ty, &deserializer_name);

            quote! {
                #inner_vector_deserializer

                let vec_passing_size = 2;
                let mut arg: Vec<u32> = Vec::from_raw_parts(offset as _, (vec_passing_size * size) as _, (vec_passing_size * size) as _);
                let mut result = Vec::with_capacity(arg.len());

                let mut arg = arg.into_iter();
                while let Some(offset) = arg.next() {
                    let size = arg.next().unwrap();

                    let value = #deserializer_ident(offset as _, size as _);
                    result.push(value);
                }

                result
            }
        }
        ParsedType::Record(record_name, _) => {
            let record_name_ident = crate::new_ident!(record_name);

            quote! {
                let mut arg: Vec<u32> = Vec::from_raw_parts(offset as _, size as _, size as _);
                let mut result = Vec::with_capacity(arg.len());

                for offset in arg {
                    let value = #record_name_ident::__fce_generated_deserialize(offset as _);
                    result.push(value);
                }

                result
            }
        }
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
