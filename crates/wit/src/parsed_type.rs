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

mod fn_arg;
mod fn_epilog;
mod fn_prolog;
mod foreign_mod_arg;
mod foreign_mod_epilog;
mod foreign_mod_prolog;
mod vector_utils;

pub(crate) use fn_arg::*;
pub(crate) use fn_epilog::*;
pub(crate) use fn_prolog::*;
pub(crate) use foreign_mod_prolog::*;
pub(crate) use foreign_mod_epilog::*;
pub(crate) use vector_utils::*;

use serde::Serialize;
use serde::Deserialize;
use syn::parse::Error;
use syn::spanned::Spanned;
use proc_macro2::TokenStream;
use serde::export::Formatter;

/// An internal representation of supported Rust types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParsedType {
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
    Vector(Box<ParsedType>),
    Record(String), // short type name
}

impl ParsedType {
    pub fn from_type(input_type: &syn::Type) -> syn::Result<Self> {
        use quote::ToTokens;

        // parse generic param T in Vec<T> to syn::Type
        fn parse_vec_bracket(args: &syn::PathArguments) -> syn::Result<&syn::Type> {
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
            match arg {
                syn::GenericArgument::Type(ty) => Ok(ty),
                _ => Err(Error::new(
                    arg.span(),
                    "Unsuitable type in Vec brackets - only Vec<u8> is supported",
                )),
            }
        }

        let path = match input_type {
            syn::Type::Path(path) => Ok(&path.path),
            _ => Err(Error::new(
                input_type.span(),
                "Incorrect argument type - passing only by value is supported now",
            )),
        }?;

        let type_segment = path
            .segments
            // argument can be given in full path form: ::std::string::String
            // that why the last one used
            .last()
            .ok_or_else(|| Error::new(path.span(), "Type should be specified"))?;

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
            "f64" => Ok(ParsedType::F64),
            "bool" => Ok(ParsedType::Boolean),
            "String" => Ok(ParsedType::Utf8String),
            "Vec" => {
                let vec_type = parse_vec_bracket(&type_segment.arguments)?;
                let parsed_type = ParsedType::from_type(vec_type)?;

                Ok(ParsedType::Vector(Box::new(parsed_type)))
            }
            _ if !type_segment.arguments.is_empty() => Err(Error::new(
                type_segment.span(),
                "type with lifetimes or generics aren't allowed".to_string(),
            )),
            _ => Ok(ParsedType::Record(
                (&type_segment.ident).into_token_stream().to_string(),
            )),
        }
    }

    pub fn from_fn_arg(fn_arg: &syn::FnArg) -> syn::Result<Self> {
        match fn_arg {
            syn::FnArg::Typed(arg) => ParsedType::from_type(&arg.ty),
            _ => Err(Error::new(
                fn_arg.span(),
                "`self` argument types aren't supported",
            )),
        }
    }

    pub fn from_return_type(ret_type: &syn::ReturnType) -> syn::Result<Option<Self>> {
        match ret_type {
            syn::ReturnType::Type(_, t) => Ok(Some(ParsedType::from_type(t.as_ref())?)),
            syn::ReturnType::Default => Ok(None),
        }
    }

    pub fn is_complex_type(&self) -> bool {
        match self {
            ParsedType::Boolean
            | ParsedType::I8
            | ParsedType::I16
            | ParsedType::I32
            | ParsedType::I64
            | ParsedType::U8
            | ParsedType::U16
            | ParsedType::U32
            | ParsedType::U64
            | ParsedType::F32
            | ParsedType::F64 => false,
            ParsedType::Utf8String | ParsedType::Vector(_) | ParsedType::Record(_) => true,
        }
    }
}

impl quote::ToTokens for ParsedType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_token_stream());
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        use quote::quote;

        match self {
            ParsedType::I8 => quote! { i8 },
            ParsedType::I16 => quote! { i16 },
            ParsedType::I32 => quote! { i32 },
            ParsedType::I64 => quote! { i64 },
            ParsedType::U8 => quote! { u8 },
            ParsedType::U16 => quote! { u16 },
            ParsedType::U32 => quote! { u32 },
            ParsedType::U64 => quote! { u64 },
            ParsedType::F32 => quote! { f32 },
            ParsedType::F64 => quote! { f64 },
            ParsedType::Boolean => quote! { bool },
            ParsedType::Utf8String => quote! { String },
            ParsedType::Vector(ty) => {
                let quoted_type = ty.to_token_stream();
                quote! { Vec<#quoted_type> }
            }
            ParsedType::Record(name) => {
                let ty = crate::new_ident!(name);
                quote! { #ty }
            }
        }
    }
}

impl std::fmt::Display for ParsedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ParsedType::Boolean => f.write_str("bool"),
            ParsedType::I8 => f.write_str("i8"),
            ParsedType::I16 => f.write_str("i16"),
            ParsedType::I32 => f.write_str("i32"),
            ParsedType::I64 => f.write_str("i64"),
            ParsedType::U8 => f.write_str("u8"),
            ParsedType::U16 => f.write_str("u16"),
            ParsedType::U32 => f.write_str("u32"),
            ParsedType::U64 => f.write_str("u64"),
            ParsedType::F32 => f.write_str("f32"),
            ParsedType::F64 => f.write_str("u64"),
            ParsedType::Utf8String => f.write_str("String"),
            ParsedType::Vector(_) => f.write_str("Vec"),
            ParsedType::Record(record_name) => f.write_str(&record_name),
        }?;

        Ok(())
    }
}
