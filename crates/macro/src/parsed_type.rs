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

use quote::quote;
use serde::Serialize;
use serde::Deserialize;
use syn::parse::Error;
use syn::spanned::Spanned;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum ParsedType {
    Empty,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Boolean,
    Utf8String,
    ByteVector,
    Record(String),
}

impl ParsedType {
    pub fn from_type(input_type: &syn::Type) -> syn::Result<Self> {
        // parses generic param T in Vec<T> to string representation
        fn parse_vec_bracket(args: &syn::PathArguments) -> syn::Result<String> {
            // checks that T is angle bracketed
            let generic_arg = match args {
                syn::PathArguments::AngleBracketed(args) => Ok(args),
                _ => Err(Error::new(
                    args.span(),
                    "It has to be a bracketed value after Vec",
                )),
            }?;

            let arg = generic_arg.args.first().ok_or_else(|| {
                Error::new(
                    generic_arg.span(),
                    "Unsuitable type in Vec brackets - only Vec<u8> is supported",
                )
            })?;

            // converts T to syn::Type
            let arg_type = match arg {
                syn::GenericArgument::Type(ty) => Ok(ty),
                _ => Err(Error::new(
                    arg.span(),
                    "Unsuitable type in Vec brackets - only Vec<u8> is supported",
                )),
            }?;

            // converts T to syn::path
            let arg_path = match arg_type {
                syn::Type::Path(path) => Ok(&path.path),
                _ => Err(Error::new(
                    arg_type.span(),
                    "Unsuitable type in Vec brackets - only Vec<u8> is supported",
                )),
            }?;

            // There could be cases like Vec<some_crate::some_module::u8>
            // that why this segments count check is needed
            if arg_path.segments.len() != 1 {
                return Err(Error::new(
                    arg_path.span(),
                    "Unsuitable type in Vec brackets - only Vec<u8> is supported",
                ));
            }

            // converts T to String
            let arg_segment = arg_path.segments.first().ok_or_else(|| {
                Error::new(
                    arg_path.span(),
                    "Unsuitable type in Vec brackets - only Vec<u8> is supported",
                )
            })?;

            Ok(arg_segment.ident.to_string())
        }

        let path = match input_type {
            syn::Type::Path(path) => Ok(&path.path),
            _ => Err(Error::new(
                input_type.span(),
                "Incorrect argument type - only Vec<u8> and String are supported",
            )),
        }?;

        let type_segment = path
            .segments
            // argument can be given in full path form: ::std::string::String
            // that why the last one used
            .last()
            .ok_or_else(|| {
                Error::new(
                    path.span(),
                    "The invocation handler should have a non-empty input argument type",
                )
            })?;

        match type_segment.ident.to_string().as_str() {
            "i8" => Ok(ParsedType::I8),
            "i16" => Ok(ParsedType::I16),
            "i32" => Ok(ParsedType::I32),
            "i64" => Ok(ParsedType::I64),
            "u8" => Ok(ParsedType::U8),
            "u16" => Ok(ParsedType::U16),
            "u32" => Ok(ParsedType::U32),
            "u64" => Ok(ParsedType::U64),
            "f32" => Ok(ParsedType::F32),
            "f64" => Ok(ParsedType::F32),
            "bool" => Ok(ParsedType::Boolean),
            "String" => Ok(ParsedType::Utf8String),
            "Vec" => match parse_vec_bracket(&type_segment.arguments) {
                Ok(value) => match value.as_str() {
                    "u8" => Ok(ParsedType::ByteVector),
                    _ => Err(Error::new(
                        value.span(),
                        "Unsuitable type in Vec brackets - only Vec<u8> is supported",
                    )),
                },
                Err(e) => Err(e),
            },
            type_name => Err(Error::new(
                type_segment.span(),
                format!("{} is unsupported", type_name),
            )),
        }
    }

    pub fn from_fn_arg(fn_arg: &syn::FnArg) -> syn::Result<Self> {
        match fn_arg {
            syn::FnArg::Typed(arg) => ParsedType::from_type(&arg.ty),
            _ => Err(Error::new(fn_arg.span(), "Unknown argument")),
        }
    }

    pub fn from_return_type(ret_type: &syn::ReturnType) -> syn::Result<Self> {
        match ret_type {
            syn::ReturnType::Type(_, t) => ParsedType::from_type(t.as_ref()),
            syn::ReturnType::Default => Ok(ParsedType::Empty),
        }
    }
}

// TODO: replace String with Ident
pub(crate) trait MacroPartsGenerator {
    fn generate_arguments(&self) -> Vec<WasmType>;

    fn generate_return_expression(&self) -> proc_macro2::TokenStream;

    fn generate_return_type(&self) -> String;

    fn generate_fn_prolog(
        &self,
        generated_arg_id: usize,
        supplied_arg_start_id: usize,
    ) -> proc_macro2::TokenStream;

    fn generate_fn_epilog(&self) -> proc_macro2::TokenStream;
}

impl MacroPartsGenerator for ParsedType {
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
