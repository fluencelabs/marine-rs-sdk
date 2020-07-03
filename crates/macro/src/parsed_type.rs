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

mod glue_code_generator;

pub(crate) use glue_code_generator::GlueCodeGenerator;

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
