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

use super::TokenStreamGenerator;
use super::GENERATED_FUNCS_PREFIX;
use super::GENERATED_SECTION_NAME;
use super::GENERATED_SECTION_PREFIX;
use crate::fce_ast_types;

use proc_macro2::TokenStream;
use quote::quote;

impl TokenStreamGenerator for fce_ast_types::AstExternModItem {
    fn generate_token_stream(self) -> syn::Result<TokenStream> {
        let data = serde_json::to_string(&self).unwrap();
        let data_size = data.len();

        let prefix = GENERATED_FUNCS_PREFIX;
        let section_name = GENERATED_SECTION_NAME;
        let section_prefix = GENERATED_SECTION_PREFIX;
        let generated_global_name = uuid::Uuid::new_v4().to_string();
        let visibility = "pub";

        let wasm_import_module_name = self.namespace;

        let glue_code = quote! {
            #[link(wasm_import_module = #wasm_import_module_name)]
            #[cfg(target_arch = "wasm32")]
            extern "C" {
                fn #import_name(#(#raw_args),*) #import_ret_type;
            }

            #[cfg(target_arch = "wasm32")]
            unsafe fn #glue_import_name(#(#args),*) #glue_ret_type {
                #prolog

                #import_name();
            }

            #[cfg_attr(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #visibility unsafe fn #prefix#func_name(#(#args)*) #return_type {
                // arg conversations from Rust types to Wasm types
                #prolog

                // calling the original function with converted args
                #return_expression #func_name(#(#raw_args)*);

                // return value conversation from Wasm type to a Rust type
                #epilog
            }

            #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #[link_section = #section_name]
            pub static #func_name#section_prefix#generated_global_name: [u8; #data_size] = { #data };
        };

        Ok(glue_code)
    }
}
