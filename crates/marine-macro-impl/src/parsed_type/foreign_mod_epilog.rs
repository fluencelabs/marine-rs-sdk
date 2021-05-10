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
                String::from_raw_parts(
                    fluence::internal::get_result_ptr() as _,
                    fluence::internal::get_result_size() as _,
                    fluence::internal::get_result_size() as _
                )
            },
            Some(ParsedType::Vector(ty, _)) => {
                let generated_der_name = "__m_generated_vec_deserializer";
                let generated_der_ident = new_ident!(generated_der_name);
                let vector_deserializer =
                    super::vector_ser_der::generate_vector_der(ty, generated_der_name);

                quote! {
                    #vector_deserializer
                    #generated_der_ident(
                        fluence::internal::get_result_ptr() as _,
                        fluence::internal::get_result_size() as _,
                    )
                }
            }
            Some(ParsedType::Record(record_name, _)) => {
                let record_ident = new_ident!(record_name);

                quote! {
                    #record_ident::__m_generated_deserialize(fluence::internal::get_result_ptr() as _)
                }
            }
            _ => panic!(
                "perhaps new type's been added to ParsedType, and this match became incomplete"
            ),
        }
    }
}
