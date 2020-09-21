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
use quote::quote;

pub(crate) fn generate_vector_serializer(
    value_ty: &ParsedType,
    arg_name: &str,
) -> proc_macro2::TokenStream {
    let values_serializer = match value_ty {
        ParsedType::Boolean
        | ParsedType::I8
        | ParsedType::I16
        | ParsedType::I32
        | ParsedType::U8
        | ParsedType::U16
        | ParsedType::U32 => {
            quote! {
                let mut result: Vec<u32> = Vec::with_capacity(arg.len());
                for value in arg {
                    result.push(value as _);
                }

                fluence::internal::transmute_vec::<u64, u8>(result)
            }
        }
        ParsedType::I64 | ParsedType::U64 => {
            quote! {
                let mut result: Vec<u64> = Vec::with_capacity(arg.len());
                for value in arg {
                    result.push(value as _);
                }

                fluence::internal::transmute_vec::<u64, u8>(result)
            }
        }
        ParsedType::F32 => {
            quote! {
                let mut result: Vec<u32> = Vec::with_capacity(arg.len());
                for value in arg {
                    result.push(value.to_bits());
                }

                fluence::internal::transmute_vec::<u32, u8>(result)
            }
        }
        ParsedType::F64 => {
            quote! {
                let mut result: Vec<u64> = Vec::with_capacity(arg.len());
                for value in arg {
                    result.push(value.to_bits());
                }

                fluence::internal::transmute_vec::<u64, u8>(result)
            }
        }
        ParsedType::Utf8String => {
            quote! {
                let mut result: Vec<u32> = Vec::with_capacity(arg.len());

                for value in arg {
                    result.push(value.as_ptr() as _);
                    result.push(value.len() as _);
                }

                fluence::internal::transmute_vec::<u32, u8>(result)
            }
        }
        ParsedType::Vector(ty) => {
            let serializer_name = format!("{}_{:?}", arg_name, ty);
            let inner_vector_serializer = generate_vector_serializer(&*ty, &serializer_name);
            let serializer_ident = crate::new_ident!(serializer_name);

            quote! {
                #inner_vector_serializer

                let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
                for value in arg {
                    result.push(#serializer_ident(arg, &serializer_name));
                }

                fluence::internal::transmute_vec::<u32, u8>(result)
            }
        }

        ParsedType::Record(record_name) => {
            let record_ident = crate::new_ident!(record_name);

            quote! {
                let mut result: Vec<u32> = Vec::with_capacity(arg.len());

                for value in arg {
                    result.push(#record_ident.__fce_generated_serialize() as _);
                }

                fluence::internal::transmute_vec::<u32, u8>(result)
            }
        }
    };

    let arg = crate::new_ident!(arg_name);

    quote! {
        fn serialize_vector_#arg(arg: Vec<#value_ty>) -> Vec<u8> {
            std::mem::forget(arg);

            if arg.is_empty() {
                return vec![];
            }

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
        ParsedType::F32 => {
            quote! {
                let arg = fluence::internal::transmute_vec::<u8, u32>().unwrap();
                arg.iter().map(f32::from_bits).collect::<Vec<_>>()
            }
        }
        ParsedType::F64 => {
            quote! {
                let arg = fluence::internal::transmute_vec::<u8, u64>().unwrap();
                arg.iter().map(f64::from_bits).collect::<Vec<_>>()
            }
        }
        ParsedType::Utf8String => {
            quote! {
                let arg = fluence::internal::transmute_vec::<u8, u32>().unwrap();
                let arg = arg.iter();
                let mut result = Vec::with_capacity(arg.len() / 2);
                while let Some(offset) = arg.next() {
                    let size = arg.next().unwrap();
                    let value = String::from_raw_parts(offset as _, size as _, size as _);
                    result.push(value);
                }

                result
            }
        }
        ParsedType::Vector(ty) => {
            let deserializer_name = format!("{}_{:?}", arg_name, ty);
            let inner_vector_deserializer = generate_vector_deserializer(&*ty, &deserializer_name);
            let deserializer_ident = crate::new_ident!(deserializer_name);

            quote! {
                #inner_vector_deserializer

                let arg = fluence::internal::transmute_vec::<u8, u32>().unwrap();
                let mut result = Vec::with_capacity(arg.len());

                for offset in arg.iter() {
                    let value = #deserializer_ident(offset, &deserializer_name);
                    result.push(value);
                }

                result
            }
        }
        ParsedType::Record(record_name) => {
            quote! {
                let arg = fluence::internal::transmute_vec::<u8, u32>().unwrap();
                let mut result = Vec::with_capacity(arg.len());

                for offset in arg {
                    let value = #record_name.__fce_generated_deserialize(arg.as_ptr());
                    result.push(value);
                }

                result
            }
        }
        v => {
            quote! {
                fluence::internal::transmute_vec::<u8, #v>().unwrap()
            }
        }
    };

    quote! {
        fn deserialize_vector_#arg(offset: u32, size: u32) -> Vec<#value_ty> {
            let arg: Vec<u8> = Vec::from_raw_parts(offset as _, size as _, size _);
            if arg.is_empty() {
                return vec![];
            }

            #values_deserializer
        }
    }
}
