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

use crate::wasm_type::WasmType;
use super::ParsedType;

use quote::quote;

pub(crate) trait GlueCodeGenerator {
    fn generate_arguments(&self) -> Vec<WasmType>;

    fn generate_return_expression(&self) -> proc_macro2::TokenStream;

    // TODO: replace String with Ident
    fn generate_return_type(&self) -> String;

    fn generate_fn_prolog(
        &self,
        generated_arg_id: usize,
        supplied_arg_start_id: usize,
    ) -> proc_macro2::TokenStream;

    fn generate_fn_epilog(&self) -> proc_macro2::TokenStream;
}

impl GlueCodeGenerator for ParsedType {
    fn generate_arguments(&self) -> Vec<WasmType> {
        // TODO: investigate possible issues in conversion between signed and unsigned types
        match self {
            ParsedType::Empty => vec![],
            ParsedType::I8 => vec![WasmType::I32],
            ParsedType::I16 => vec![WasmType::I32],
            ParsedType::I32 => vec![WasmType::I32],
            ParsedType::I64 => vec![WasmType::I64],
            ParsedType::U8 => vec![WasmType::I32],
            ParsedType::U16 => vec![WasmType::I32],
            ParsedType::U32 => vec![WasmType::I32],
            ParsedType::U64 => vec![WasmType::I64],
            ParsedType::F32 => vec![WasmType::F32],
            ParsedType::F64 => vec![WasmType::F64],
            ParsedType::Boolean => vec![WasmType::I32],
            ParsedType::Utf8String => vec![WasmType::I32, WasmType::I32],
            ParsedType::ByteVector => vec![WasmType::I32, WasmType::I32],
            ParsedType::Record(_) => vec![WasmType::I32, WasmType::I32],
        }
    }

    fn generate_return_expression(&self) -> proc_macro2::TokenStream {
        match self {
            ParsedType::Empty => quote! {},
            ParsedType::Utf8String => quote! {},
            ParsedType::ByteVector => quote! {},
            ParsedType::Record(_) => quote! {},
            _ => quote! {
                let result =
            },
        }
    }

    fn generate_return_type(&self) -> String {
        match self {
            ParsedType::I8 => "-> i32",
            ParsedType::I16 => "-> i32",
            ParsedType::I32 => "-> i32",
            ParsedType::I64 => "-> i64",
            ParsedType::U8 => "-> i32",
            ParsedType::U16 => "-> i32",
            ParsedType::U32 => "-> i32",
            ParsedType::U64 => "-> i64",
            ParsedType::F32 => "-> f32",
            ParsedType::F64 => "-> f64",
            ParsedType::Boolean => "-> i32",
            _ => "",
        }
        .to_string()
    }

    fn generate_fn_prolog(
        &self,
        generated_ard_id: usize,
        supplied_arg_start_id: usize,
    ) -> proc_macro2::TokenStream {
        match self {
            ParsedType::Empty => unimplemented!(),
            ParsedType::I8 => quote! {
                let converted_arg_#generated_ard_id = arg_#supplied_arg_start_id as i8;
            },
            ParsedType::I16 => quote! {
                let converted_arg_#generated_ard_id = arg_#supplied_arg_start_id as i16;
            },
            ParsedType::I32 => quote! {
                let converted_arg_#generated_ard_id = arg_#supplied_arg_start_id as i32;
            },
            ParsedType::I64 => quote! {
                let converted_arg_#generated_ard_id = arg_#supplied_arg_start_id as i64;
            },
            ParsedType::U8 => quote! {
                let converted_arg_#generated_ard_id = arg_#supplied_arg_start_id as u8;
            },
            ParsedType::U16 => quote! {
                let converted_arg_#generated_ard_id = arg_#supplied_arg_start_id as u16;
            },
            ParsedType::U32 => quote! {
                let converted_arg_#generated_ard_id = arg_#supplied_arg_start_id as u32;
            },
            ParsedType::U64 => quote! {
                let converted_arg_#generated_ard_id = arg_#supplied_arg_start_id as u64;
            },
            ParsedType::F32 => quote! {
                let converted_arg_#generated_ard_id = arg_#supplied_arg_start_id as f32;
            },
            ParsedType::F64 => quote! {
                let converted_arg_#generated_ard_id = arg_#supplied_arg_start_id as f64;
            },
            ParsedType::Boolean => quote! {
                let converted_arg_#generated_ard_id = arg_#supplied_arg_start_id as bool;
            },
            ParsedType::Utf8String => quote! {
                let converted_arg_#generated_ard_id = String::from_raw_parts(
                                                        arg_#supplied_arg_start_id,
                                                        arg_#(supplied_arg_start_id+1),
                                                        arg_#(supplied_arg_start_id+1)
                                                      );
            },
            ParsedType::ByteVector => quote! {
                let converted_arg_#generated_ard_id = Vec::from_raw_parts(
                                                        arg_#supplied_arg_start_id,
                                                        arg_#(supplied_arg_start_id+1),
                                                        arg_#(supplied_arg_start_id+1)
                                                      );
            },
            ParsedType::Record(record_name) => quote! {
                let converted_arg_#generated_ard_id = __fce_generated_converter_#record_name(
                                                        arg_#supplied_arg_start_id,
                                                        arg_#(supplied_arg_start_id+1)
                                                        );
            },
        }
    }

    fn generate_fn_epilog(&self) -> proc_macro2::TokenStream {
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
                fluence::set_result_ptr(result.as_ptr() as _);
                fluence::set_result_size(result.len() as _);
                std::mem::forget(result);
            },
            ParsedType::ByteVector => quote! {
                fluence::set_result_ptr(result.as_ptr() as _);
                fluence::set_result_size(result.len() as _);
                std::mem::forget(result);
            },
            ParsedType::Record(_) => quote! {
                fluence::set_result_ptr(result.as_ptr() as _);
                fluence::set_result_size(result.len() as _);
                std::mem::forget(result);
            },
        }
    }
}
