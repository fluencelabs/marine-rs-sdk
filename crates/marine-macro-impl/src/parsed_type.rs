/*
 * Marine Rust SDK
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod fn_arg;
mod fn_epilog;
mod fn_prolog;
mod foreign_mod_arg;
mod foreign_mod_epilog;
mod foreign_mod_prolog;
mod traits;
mod utils;
mod vector_ser_der;

pub(crate) use fn_arg::*;
pub(crate) use fn_epilog::*;
pub(crate) use fn_prolog::*;
pub(crate) use foreign_mod_prolog::*;
pub(crate) use foreign_mod_epilog::*;
pub(crate) use utils::*;
pub(crate) use vector_ser_der::*;

use serde::Serialize;
use serde::Deserialize;
use syn::parse::Error;
use syn::spanned::Spanned;

/// An internal representation of supported Rust types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParsedType {
    I8(PassingStyle),
    I16(PassingStyle),
    I32(PassingStyle),
    I64(PassingStyle),
    U8(PassingStyle),
    U16(PassingStyle),
    U32(PassingStyle),
    U64(PassingStyle),
    F32(PassingStyle),
    F64(PassingStyle),
    Boolean(PassingStyle),
    Utf8Str(PassingStyle),
    Utf8String(PassingStyle),
    Vector(Box<ParsedType>, PassingStyle),
    Record(String, PassingStyle), // short type name
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassingStyle {
    ByValue,
    ByRef,
    ByMutRef,
}

impl ParsedType {
    pub fn from_type(input_type: &syn::Type) -> syn::Result<Self> {
        use quote::ToTokens;

        let (path, passing_style) = type_to_path_passing_style(input_type)?;

        let type_segment = path
            .segments
            // argument can be given in full path form: ::std::string::String
            // that why the last one used
            .last()
            .ok_or_else(|| Error::new(path.span(), "Type should be specified"))?;

        match type_segment.ident.to_string().as_str() {
            "i8" => Ok(ParsedType::I8(passing_style)),
            "i16" => Ok(ParsedType::I16(passing_style)),
            "i32" => Ok(ParsedType::I32(passing_style)),
            "i64" => Ok(ParsedType::I64(passing_style)),
            "u8" => Ok(ParsedType::U8(passing_style)),
            "u16" => Ok(ParsedType::U16(passing_style)),
            "u32" => Ok(ParsedType::U32(passing_style)),
            "u64" => Ok(ParsedType::U64(passing_style)),
            "f32" => Ok(ParsedType::F32(passing_style)),
            "f64" => Ok(ParsedType::F64(passing_style)),
            "bool" => Ok(ParsedType::Boolean(passing_style)),
            "str" => Ok(ParsedType::Utf8Str(passing_style)),
            "String" => Ok(ParsedType::Utf8String(passing_style)),
            "Vec" => {
                let vec_type = parse_vec_bracket(&type_segment.arguments)?;
                let parsed_type = ParsedType::from_type(vec_type)?;

                Ok(ParsedType::Vector(Box::new(parsed_type), passing_style))
            }
            _ if !type_segment.arguments.is_empty() => Err(Error::new(
                type_segment.span(),
                "types with lifetimes or generics aren't allowed".to_string(),
            )),
            _ => Ok(ParsedType::Record(
                (&type_segment.ident).into_token_stream().to_string(),
                passing_style,
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
            ParsedType::Boolean(_)
            | ParsedType::I8(_)
            | ParsedType::I16(_)
            | ParsedType::I32(_)
            | ParsedType::I64(_)
            | ParsedType::U8(_)
            | ParsedType::U16(_)
            | ParsedType::U32(_)
            | ParsedType::U64(_)
            | ParsedType::F32(_)
            | ParsedType::F64(_) => false,
            ParsedType::Utf8Str(_)
            | ParsedType::Utf8String(_)
            | ParsedType::Vector(..)
            | ParsedType::Record(..) => true,
        }
    }
}

fn type_to_path_passing_style(input_type: &syn::Type) -> syn::Result<(&syn::Path, PassingStyle)> {
    match input_type {
        syn::Type::Path(path) => Ok((&path.path, PassingStyle::ByValue)),
        syn::Type::Reference(type_reference) => match &*type_reference.elem {
            syn::Type::Path(path) => {
                let passing_style = match type_reference.mutability {
                    Some(_) => PassingStyle::ByMutRef,
                    None => PassingStyle::ByRef,
                };

                Ok((&path.path, passing_style))
            }
            _ => Err(Error::new(
                input_type.span(),
                "Incorrect argument type, only path is available on this position",
            )),
        },
        _ => Err(Error::new(
            input_type.span(),
            "Incorrect argument type, only path or reference are available on this position",
        )),
    }
}

// parse generic param T in Vec<T> to syn::Type
fn parse_vec_bracket(args: &syn::PathArguments) -> syn::Result<&syn::Type> {
    // checks that T is angle bracketed
    let generic_arg = match args {
        syn::PathArguments::AngleBracketed(args) => Ok(args),
        _ => Err(Error::new(
            args.span(),
            "expected value in angle brackets (<>)",
        )),
    }?;

    let arg = generic_arg.args.first().ok_or_else(|| {
        Error::new(
            generic_arg.span(),
            "Invalid type in Vec brackets. (NOTE: lifetimes, bindings, constraints and consts are not supported)",
        )
    })?;

    // converts T to syn::Type
    match arg {
        syn::GenericArgument::Type(ty) => Ok(ty),
        _ => Err(Error::new(
            arg.span(),
            "Invalid type in Vec brackets. (NOTE: lifetimes, bindings, constraints and consts are not supported)",
        )),
    }
}
