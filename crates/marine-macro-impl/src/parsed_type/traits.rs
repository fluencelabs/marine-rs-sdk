/*
 * Fluence Marine Rust SDK
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

use super::PassingStyle;
use super::ParsedType;

use quote::quote;
use proc_macro2::TokenStream;

use std::fmt;

impl quote::ToTokens for PassingStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_token_stream());
    }

    fn to_token_stream(&self) -> TokenStream {
        match self {
            PassingStyle::ByValue => quote! {},
            PassingStyle::ByRef => quote! { & },
            PassingStyle::ByMutRef => quote! { &mut },
        }
    }
}

impl quote::ToTokens for ParsedType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_token_stream());
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            ParsedType::I8(passing_style) => quote! { #passing_style i8 },
            ParsedType::I16(passing_style) => quote! { #passing_style i16 },
            ParsedType::I32(passing_style) => quote! { #passing_style i32 },
            ParsedType::I64(passing_style) => quote! { #passing_style i64 },
            ParsedType::U8(passing_style) => quote! { #passing_style u8 },
            ParsedType::U16(passing_style) => quote! { #passing_style u16 },
            ParsedType::U32(passing_style) => quote! { #passing_style u32 },
            ParsedType::U64(passing_style) => quote! { #passing_style u64 },
            ParsedType::F32(passing_style) => quote! { #passing_style f32 },
            ParsedType::F64(passing_style) => quote! { #passing_style f64 },
            ParsedType::Boolean(passing_style) => quote! { #passing_style bool },
            ParsedType::Utf8Str(passing_style) => quote! { #passing_style str },
            ParsedType::Utf8String(passing_style) => quote! { #passing_style String },
            ParsedType::Vector(ty, passing_style) => {
                let quoted_type = ty.to_token_stream();
                quote! { #passing_style Vec<#quoted_type> }
            }
            ParsedType::Record(name, passing_style) => {
                let ty = crate::new_ident!(name);
                quote! { #passing_style #ty }
            }
        }
    }
}

impl fmt::Display for ParsedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ParsedType::Boolean(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("bool")
            }
            ParsedType::I8(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("i8")
            }
            ParsedType::I16(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("i16")
            }
            ParsedType::I32(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("i32")
            }
            ParsedType::I64(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("i64")
            }
            ParsedType::U8(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("u8")
            }
            ParsedType::U16(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("u16")
            }
            ParsedType::U32(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("u32")
            }
            ParsedType::U64(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("u64")
            }
            ParsedType::F32(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("f32")
            }
            ParsedType::F64(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("u64")
            }
            ParsedType::Utf8Str(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("str")
            }
            ParsedType::Utf8String(passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("String")
            }
            ParsedType::Vector(ty, passing_style) => {
                passing_style.fmt(f)?;
                f.write_str("Vec<")?;
                ty.fmt(f)?;
                f.write_str(">")
            }
            ParsedType::Record(record_name, passing_style) => {
                passing_style.fmt(f)?;
                f.write_str(record_name)
            }
        }?;

        Ok(())
    }
}

impl fmt::Display for PassingStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            PassingStyle::ByValue => Ok(()),
            PassingStyle::ByRef => f.write_str("&"),
            PassingStyle::ByMutRef => f.write_str("&mut"),
        }
    }
}
