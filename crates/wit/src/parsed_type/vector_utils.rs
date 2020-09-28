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
        ParsedType::Boolean => {
            quote! {
                unimplemented!()
            }
        }
        ParsedType::I8 | ParsedType::U8 => {
            quote! {
                let arg = std::mem::ManuallyDrop::new(arg);
                (arg.as_ptr() as _, arg.len() as _)
            }
        }
        ParsedType::I16 | ParsedType::U16 => {
            quote! {
                let arg = std::mem::ManuallyDrop::new(arg);
                (arg.as_ptr() as _, (2 * arg.len()) as _)
            }
        }
        ParsedType::I32 | ParsedType::U32 => {
            quote! {
                let arg = std::mem::ManuallyDrop::new(arg);
                (arg.as_ptr() as _, (4 * arg.len()) as _)
            }
        }
        ParsedType::I64 | ParsedType::U64 => {
            quote! {
                let arg = std::mem::ManuallyDrop::new(arg);
                (arg.as_ptr() as _, (8 * arg.len()) as _)
            }
        }
        ParsedType::F32 => {
            quote! {
                let mut result: Vec<u32> = Vec::with_capacity(arg.len());
                for value in arg {
                    result.push(value.to_bits());
                }

                let result = std::mem::ManuallyDrop::new(result);
                (result.as_ptr() as _, (4 * result.len()) as _)
            }
        }
        ParsedType::F64 => {
            quote! {
                let mut result: Vec<u64> = Vec::with_capacity(arg.len());
                for value in arg {
                    result.push(value.to_bits());
                }

                let result = std::mem::ManuallyDrop::new(result);
                (result.as_ptr() as _, (8 * result.len()) as _)
            }
        }
        ParsedType::Utf8String => {
            quote! {
                let mut result: Vec<u32> = Vec::with_capacity(arg.len());

                for value in arg {
                    let value = std::mem::ManuallyDrop::new(value);
                    result.push(value.as_ptr() as _);
                    result.push(value.len() as _);
                }

                let result = std::mem::ManuallyDrop::new(result);
                (result.as_ptr() as _, (4 * result.len()) as _)
            }
        }
        ParsedType::Vector(ty) => {
            let serializer_name = format!("{}_{}", arg_name, ty);
            let inner_vector_serializer = generate_vector_serializer(&*ty, &serializer_name);
            let serializer_ident = crate::new_ident!(serializer_name);

            quote! {
                #inner_vector_serializer

                let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
                for value in arg {
                    let (ptr, size) = #serializer_ident(value);
                    result.push(ptr as _);
                    result.push(size as _);
                }

                let result = std::mem::ManuallyDrop::new(result);
                (result.as_ptr() as _, (4 * result.len()) as _)
            }
        }

        ParsedType::Record(_) => {
            quote! {
                let mut result: Vec<u32> = Vec::with_capacity(arg.len());

                for value in arg {
                    result.push(value.__fce_generated_serialize() as _);
                }

                let result = std::mem::ManuallyDrop::new(result);
                (result.as_ptr() as _, (4 * result.len()) as _)
            }
        }
    };

    let arg = crate::new_ident!(arg_name);

    quote! {
        unsafe fn #arg(arg: Vec<#value_ty>) -> (u32, u32) {
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
        ParsedType::Boolean => {
            quote! {
                unimplemented!()
            }
        }
        ParsedType::F32 => {
            quote! {
                let mut arg: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
                let mut result = Vec::with_capacity(arg.len());

                for value in arg {
                    result.push(f32::from_bits(value as _));
                }

                result
            }
        }
        ParsedType::F64 => {
            quote! {
                let mut arg: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
                let mut result = Vec::with_capacity(arg.len());

                for value in arg {
                    result.push(f64::from_bits(value as _));
                }

                result
            }
        }
        ParsedType::Utf8String => {
            quote! {
                let mut arg: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
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
        ParsedType::Vector(ty) => {
            let deserializer_name = format!("{}_{}", arg_name, ty);
            let inner_vector_deserializer = generate_vector_deserializer(&*ty, &deserializer_name);
            let deserializer_ident = crate::new_ident!(deserializer_name);

            quote! {
                #inner_vector_deserializer

                let mut arg: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
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
        ParsedType::Record(record_name) => {
            let record_name_ident = crate::new_ident!(record_name);

            quote! {
                let mut arg: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
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
                let mut arg: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
                let mut result = Vec::with_capacity(arg.len());

                for value in arg {
                    result.push(value as _);
                }

                result
            }
        }
    };

    quote! {
        unsafe fn #arg(offset: u32, size: u32) -> Vec<#value_ty> {
            let size = size / 8;
            #values_deserializer
        }
    }
}
