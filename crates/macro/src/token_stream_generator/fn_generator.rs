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
use crate::parsed_type::MacroPartsGenerator;
use super::GENERATED_FUNCS_PREFIX;
use super::GENERATED_SECTION_NAME;
use super::GENERATED_SECTION_PREFIX;
use super::TokenStreamGenerator;

use proc_macro2::TokenStream;
use quote::quote;
use crate::wasm_type::WasmType;

impl TokenStreamGenerator for fce_ast_types::AstFunctionItem {
    fn generate_token_stream(self) -> syn::Result<TokenStream> {
        let data = serde_json::to_string(&self).unwrap();
        let data_size = data.len();
        let func_name = self.name;

        let prefix = GENERATED_FUNCS_PREFIX;
        let section_name = GENERATED_SECTION_NAME;
        let section_prefix = GENERATED_SECTION_PREFIX;
        let generated_global_name = uuid::Uuid::new_v4().to_string();

        let return_type = self.output_type.generate_return_type();
        let return_expression = self.output_type.generate_return_expression();
        let epilog = self.output_type.generate_fn_epilog();

        let mut prolog = TokenStream::new();
        let mut args: Vec<String> = Vec::with_capacity(self.input_types.len());
        let mut raw_args: Vec<WasmType> = Vec::with_capacity(self.input_types.len());
        let mut input_type_id = 0;
        for input_type in self.input_types {
            let type_prolog = input_type.generate_fn_prolog(input_type_id, input_type_id);
            let type_raw_args = input_type.generate_arguments();

            args.extend(
                type_raw_args
                    .iter()
                    .enumerate()
                    .map(|(id, _)| format!("converted_arg_{}", input_type_id + id)),
            );

            input_type_id += type_raw_args.len();
            raw_args.extend(type_raw_args);
            prolog.extend(type_prolog);
        }

        let embedded_tokens = quote! {
            #[cfg_attr(
                target_arch = "wasm32",
                export_name = #func_name
            )]
            #[doc(hidden)]
            #[allow(clippy::all)]
            pub extern "C" fn #prefix#func_name(#(#raw_args)*) #return_type {
                #prolog

                #return_expression #func_name(#(#args)*);

                #epilog
            }

            #[cfg(target_arch = "wasm32")]
            #[allow(clippy::all)]
            #[doc(hidden)]
            #[link_section = #section_name]
            pub static #func_name#section_prefix#generated_global_name: [u8; #data_size] = { #data };
        };

        Ok(embedded_tokens)
    }
}
