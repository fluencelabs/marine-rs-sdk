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

use super::ParsedType;
use crate::new_ident;

use quote::quote;

/// This trait could be used to generate various parts needed to construct epilog of an wrapper of
/// import function.
pub(crate) trait ForeignModEpilogGlueCodeGenerator {
    fn generate_wrapper_return_type(&self) -> proc_macro2::TokenStream;

    fn generate_wrapper_epilog(&self) -> proc_macro2::TokenStream;
}

impl ForeignModEpilogGlueCodeGenerator for Option<ParsedType> {
    fn generate_wrapper_return_type(&self) -> proc_macro2::TokenStream {
        use quote::ToTokens;

        match self {
            Some(ty) => {
                let ty = ty.to_token_stream();
                quote! { -> #ty }
            }
            None => quote!(),
        }
    }

    fn generate_wrapper_epilog(&self) -> proc_macro2::TokenStream {
        match self {
            None => quote!(),
            Some(ParsedType::Boolean(_)) => quote! {
                return result != 0;
            },
            Some(ty) if !ty.is_complex_type() => quote! {
                return result as _;
            },
            Some(ParsedType::Utf8String(_)) => quote! {
                let ptr = marine_rs_sdk::internal::get_result_ptr();
                let size = marine_rs_sdk::internal::get_result_size();
                // Empty string has a non-zero buffer address in Rust,
                // so we ensure that an empty string is correctly represented.
                match size {
                    0 => String::default(),
                    _ => String::from_raw_parts(ptr as _, size as _, size as _)
                }
            },
            Some(ParsedType::Vector(ty, _)) => {
                let generated_der_name = "__m_generated_vec_deserializer";
                let generated_der_ident = new_ident!(generated_der_name);
                let vector_deserializer =
                    super::vector_ser_der::generate_vector_der(ty, generated_der_name);

                quote! {
                    #vector_deserializer
                    #generated_der_ident(
                        marine_rs_sdk::internal::get_result_ptr() as _,
                        marine_rs_sdk::internal::get_result_size() as _,
                    )
                }
            }
            Some(ParsedType::Record(record_name, _)) => {
                let record_ident = new_ident!(record_name);

                quote! {
                    #record_ident::__m_generated_deserialize(marine_rs_sdk::internal::get_result_ptr() as _)
                }
            }
            _ => panic!(
                "perhaps new type's been added to ParsedType, and this match became incomplete"
            ),
        }
    }
}
