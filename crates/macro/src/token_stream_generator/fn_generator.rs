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

use crate::fce_ast_types;
use crate::parsed_type::ArgumentsGenerator;
use crate::parsed_type::EpilogGenerator;
use crate::parsed_type::PrologGenerator;
use super::GENERATED_FUNCS_PREFIX;
use super::TokenStreamGenerator;

use proc_macro2::TokenStream;
use quote::quote;

impl TokenStreamGenerator for fce_ast_types::AstFunctionItem {
    fn generate_token_stream(self) -> syn::Result<TokenStream> {
        let func_name = self.name;
        let prefix = "__fce_generated_func_";

        let embedded_tokens = quote! {
            #[cfg_attr(
                target_arch = "wasm32",
                export_name = #func_name
            )]
            #[doc(hidden)]
            #[allow(clippy::all)]
            pub extern "C" fn #prefix_#func_name(#(#raw_args)*) #ret_type {
                #prolog

                #ret_expression #func_name(#(#args)*);

                #epilog
            }

            #[cfg(target_arch = "wasm32")]
            #[allow(clippy::all)]
            #[doc(hidden)]
            #[link_section = #section_name]
            pub static #generated_global_name: [u8; #size] = { #data };
        };

        Ok(embedded_tokens)
    }
}
