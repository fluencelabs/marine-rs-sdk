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

use super::GENERATED_FUNCS_PREFIX;
use super::GENERATED_SECTION_NAME;
use super::GENERATED_SECTION_PREFIX;
use super::TokenStreamGenerator;
use crate::fce_ast_types;
use crate::parsed_type::FnGlueCodeGenerator;

use proc_macro2::TokenStream;
use quote::quote;
use crate::wasm_type::WasmType;

impl TokenStreamGenerator for fce_ast_types::AstFunctionItem {
    fn generate_token_stream(self) -> syn::Result<TokenStream> {
        // TODO: change serialization protocol
        let fce_type = fce_ast_types::FCEAst::Function(self.clone());
        let data = serde_json::to_vec(&fce_type).unwrap();
        let data_size = data.len();
        let data = syn::LitByteStr::new(&data, proc_macro2::Span::call_site());

        let signature = self.signature;

        let func_name =
            syn::parse_str::<syn::Ident>(&(GENERATED_FUNCS_PREFIX.to_string() + &signature.name))?;
        let original_func_ident = syn::parse_str::<syn::Ident>(&signature.name)?;
        //let func_name = syn::parse_str::<syn::Ident>(&(GENERATED_FUNCS_PREFIX.to_string() + &self.name))?;

        let section_name = GENERATED_SECTION_NAME.to_string() + &signature.name.replace("-", "_");
        let export_func_name = signature.name;

        //let section_prefix = syn::parse_str::<syn::Ident>(GENERATED_SECTION_PREFIX)?;
        let global_static_name = syn::parse_str::<syn::Ident>(
            &(GENERATED_SECTION_PREFIX.to_string() + &export_func_name),
        )
        .unwrap();

        let return_type = signature.output_type.generate_fn_sig_return_expression();
        let return_expression = signature.output_type.generate_return_expression();
        let epilog = signature.output_type.generate_fn_epilog();

        let mut prolog = TokenStream::new();
        let mut args: Vec<syn::Ident> = Vec::with_capacity(signature.input_types.len());

        let mut raw_arg_names = Vec::with_capacity(signature.input_types.len());
        let mut raw_arg_types: Vec<WasmType> = Vec::with_capacity(signature.input_types.len());
        let mut input_type_id = 0;

        for input_type in signature.input_types {
            let type_prolog = input_type.generate_fn_prolog(input_type_id, input_type_id);
            let type_raw_args = input_type.generate_arguments();

            args.push(
                syn::parse_str::<syn::Ident>(&format!("converted_arg_{}", input_type_id)).unwrap(),
            );

            raw_arg_names.extend(type_raw_args.iter().enumerate().map(|(id, _)| {
                syn::parse_str::<syn::Ident>(&format!("arg_{}", input_type_id + id)).unwrap()
            }));

            input_type_id += type_raw_args.len();
            raw_arg_types.extend(type_raw_args);
            prolog.extend(type_prolog);
        }

        let original_func = self.original;

        let glue_code = quote! {
            #original_func

            #[cfg_attr(
                target_arch = "wasm32",
                export_name = #export_func_name
            )]
            #[no_mangle]
            #[doc(hidden)]
            #[allow(clippy::all)]
            pub unsafe fn #func_name(#(#raw_arg_names: #raw_arg_types),*) #return_type {
                #prolog

                // calling the original function with converted args
                #return_expression #original_func_ident(#(#args), *);

                // return value conversation from Rust type to a Wasm type
                #epilog
            }

            // #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #[link_section = #section_name]
            pub static #global_static_name: [u8; #data_size] = { *#data };
        };

        Ok(glue_code)
    }
}
