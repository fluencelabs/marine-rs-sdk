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

pub(crate) trait FnGlueCodeGenerator {
    fn generate_arguments(&self) -> Vec<WasmType>;

    fn generate_return_expression(&self) -> proc_macro2::TokenStream;

    fn generate_fn_sig_return_expression(&self) -> proc_macro2::TokenStream;

    fn generate_fn_prolog(
        &self,
        generated_arg_id: usize,
        supplied_arg_start_id: usize,
    ) -> proc_macro2::TokenStream;

    fn generate_fn_epilog(&self) -> proc_macro2::TokenStream;
}

impl FnGlueCodeGenerator for ParsedType {
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
            ParsedType::Utf8String => quote! {},
            ParsedType::ByteVector => quote! {},
            ParsedType::Record(_) => quote! {},
            _ => quote! {
                let result =
            },
        }
    }

    fn generate_fn_sig_return_expression(&self) -> proc_macro2::TokenStream {
        let ty = match self {
            ParsedType::I8 => Some("i32"),
            ParsedType::I16 => Some("i32"),
            ParsedType::I32 => Some("i32"),
            ParsedType::I64 => Some("i64"),
            ParsedType::U8 => Some("i32"),
            ParsedType::U16 => Some("i32"),
            ParsedType::U32 => Some("i32"),
            ParsedType::U64 => Some("i64"),
            ParsedType::F32 => Some("f32"),
            ParsedType::F64 => Some("f64"),
            ParsedType::Boolean => Some("i32"),
            _ => None,
        };

        match ty {
            Some(ty) => {
                let ty = syn::Ident::new(ty, proc_macro2::Span::call_site());
                quote! { -> #ty}
            }
            None => quote! {},
        }
    }

    fn generate_fn_prolog(
        &self,
        generated_arg_id: usize,
        supplied_arg_start_id: usize,
    ) -> proc_macro2::TokenStream {
        let generated_arg_id = syn::Ident::new(
            &format!("converted_arg_{}", generated_arg_id),
            proc_macro2::Span::call_site(),
        );

        match self {
            ParsedType::Empty => unimplemented!(),
            ParsedType::I8 => {
                let supplied_arg_start_id = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let #generated_arg_id = #supplied_arg_start_id as i8;
                }
            }
            ParsedType::I16 => {
                let supplied_arg_start_id = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let #generated_arg_id = #supplied_arg_start_id as i16;
                }
            }
            ParsedType::I32 => {
                let supplied_arg_start_id = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let #generated_arg_id = #supplied_arg_start_id as i32;
                }
            }
            ParsedType::I64 => {
                let supplied_arg_start_id = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let #generated_arg_id = #supplied_arg_start_id as i64;
                }
            }
            ParsedType::U8 => {
                let supplied_arg_start_id = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let #generated_arg_id = #supplied_arg_start_id as u8;
                }
            }
            ParsedType::U16 => {
                let supplied_arg_start_id = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let #generated_arg_id = #supplied_arg_start_id as u16;
                }
            }
            ParsedType::U32 => {
                let supplied_arg_start_id = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let #generated_arg_id = #supplied_arg_start_id as u32;
                }
            }
            ParsedType::U64 => {
                let supplied_arg_start_id = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let #generated_arg_id = #supplied_arg_start_id as u64;
                }
            }
            ParsedType::F32 => {
                let supplied_arg_start_id = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let #generated_arg_id = #supplied_arg_start_id as f32;
                }
            }
            ParsedType::F64 => {
                let supplied_arg_start_id = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let #generated_arg_id = #supplied_arg_start_id as f64;
                }
            }
            ParsedType::Boolean => {
                let supplied_arg_start_id = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    let #generated_arg_id = #supplied_arg_start_id as bool;
                }
            }
            ParsedType::Utf8String => {
                let ptr = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                let size = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id + 1),
                    proc_macro2::Span::call_site(),
                );

                quote! {
                    let #generated_arg_id = String::from_raw_parts(#ptr as _, #size as _ , #size as _);
                }
            }
            ParsedType::ByteVector => {
                let ptr = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id),
                    proc_macro2::Span::call_site(),
                );
                let size = syn::Ident::new(
                    &format!("arg_{}", supplied_arg_start_id + 1),
                    proc_macro2::Span::call_site(),
                );

                quote! {
                    let #generated_arg_id = Vec::from_raw_parts(#ptr as _, #size as _, #size as _);
                }
            }
            ParsedType::Record(record_name) => {
                quote! {
                let #generated_arg_id = __fce_generated_converter_#record_name(
                                                        arg_#supplied_arg_start_id,
                                                        arg_#(supplied_arg_start_id+1)
                                                        );
                }
            }
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
