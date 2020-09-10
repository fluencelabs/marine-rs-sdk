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

/// This trait could be used to generate raw args needed to construct a wrapper of an import
/// function.
pub(crate) trait ForeignModArgGlueCodeGenerator {
    fn generate_raw_args(&self, arg_start_id: usize) -> proc_macro2::TokenStream;
}

impl ForeignModArgGlueCodeGenerator for ParsedType {
    fn generate_raw_args(&self, arg_start_id: usize) -> proc_macro2::TokenStream {
        let arg = crate::new_ident!(format!("arg_{}", arg_start_id));

        match self {
            ParsedType::Utf8String | ParsedType::ByteVector => {
                quote! { #arg.as_ptr() as _, #arg.len() as _ }
            }
            ParsedType::Record(_) => {
                quote! {
                    #arg.__fce_generated_serialize() as _
                }
            }
            _ => quote! { #arg },
        }
    }
}
