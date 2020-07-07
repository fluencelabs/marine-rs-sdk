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
use crate::parsed_type::FnGlueCodeGenerator;
use crate::parsed_type::ForeignModeGlueCodeGenerator;

impl TokenStreamGenerator for fce_ast_types::AstExternModItem {
    fn generate_token_stream(self) -> syn::Result<TokenStream> {
        // TODO: change serialization protocol
        let fce_type = fce_ast_types::FCEAst::ExternMod(self.clone());
        let data = serde_json::to_vec(&fce_type).unwrap();
        let data_size = data.len();
        let data = syn::LitByteStr::new(&data, proc_macro2::Span::call_site());

        let global_static_name = syn::Ident::new(
            &(GENERATED_SECTION_PREFIX.to_string() + &self.namespace.replace(".", "_")),
            proc_macro2::Span::call_site(),
        );

        let section_name = GENERATED_SECTION_NAME.to_string() + &self.namespace.replace(".", "_");

        let wasm_import_module_name = &self.namespace;
        let generated_imports = generate_extern_section_items(&self);
        let wrapper_functions = generate_wrapper_functions(&self);

        let glue_code = quote! {
            #[link(wasm_import_module = #wasm_import_module_name)]
            // #[cfg(target_arch = "wasm32")]
            extern "C" {
                #generated_imports
            }

            #wrapper_functions

            // #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #[link_section = #section_name]
            pub static #global_static_name: [u8; #data_size] = { *#data };
        };

        Ok(glue_code)
    }
}

fn generate_extern_section_items(extern_item: &fce_ast_types::AstExternModItem) -> TokenStream {
    use crate::wasm_type::WasmType;

    let mut token_stream = TokenStream::new();

    for import in &extern_item.imports {
        let ret_type = import
            .signature
            .output_type
            .generate_fn_sig_return_expression();
        let link_name = import
            .link_name
            .as_ref()
            .unwrap_or_else(|| &import.signature.name);
        let import_name = generate_import_name(&import.signature.name);
        let raw_arg_types: Vec<WasmType> = import
            .signature
            .input_types
            .iter()
            .map(|input_type| input_type.generate_arguments())
            .flatten()
            .collect();
        let raw_arg_names: Vec<syn::Ident> = raw_arg_types
            .iter()
            .enumerate()
            .map(|(id, _)| syn::Ident::new(&format!("arg_{}", id), proc_macro2::Span::call_site()))
            .collect();

        let func = quote! {
            #[link_name = #link_name]
            pub fn #import_name(#(#raw_arg_names: #raw_arg_types),*) #ret_type;
        };

        token_stream.extend(func);
    }

    token_stream
}

fn generate_import_name(import_name: &str) -> syn::Ident {
    syn::Ident::new(
        &format!("{}_{}", GENERATED_FUNCS_PREFIX, import_name),
        proc_macro2::Span::call_site(),
    )
}

fn generate_wrapper_functions(extern_item: &fce_ast_types::AstExternModItem) -> TokenStream {
    let mut token_stream = TokenStream::new();

    for import in &extern_item.imports {
        let visibility = syn::Ident::new("pub", proc_macro2::Span::call_site());
        let func_name = syn::Ident::new(&import.signature.name, proc_macro2::Span::call_site());
        let arg_types: Vec<proc_macro2::TokenStream> = import
            .signature
            .input_types
            .iter()
            .map(|input_type| input_type.generate_input_type())
            .collect();
        let arg_names: Vec<syn::Ident> = arg_types
            .iter()
            .enumerate()
            .map(|(id, _)| syn::Ident::new(&format!("arg_{}", id), proc_macro2::Span::call_site()))
            .collect();
        let return_type = import
            .signature
            .output_type
            .generate_wrapper_sign_expression();
        let import_func_name = generate_import_name(&import.signature.name);
        let raw_args: Vec<proc_macro2::TokenStream> = import
            .signature
            .input_types
            .iter()
            .enumerate()
            .map(|(id, input_type)| input_type.generate_raw_args(id))
            .collect();

        let return_expression = import.signature.output_type.generate_return_expression();
        let epilog = import.signature.output_type.generate_wrapper_epilog();

        let wrapper_func = quote! {
            // #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #visibility fn #func_name(#(#arg_names: #arg_types), *) #return_type {
                unsafe {
                    // calling the original function with converted args
                    #return_expression #import_func_name(#(#raw_args), *);

                    // return value conversation from Wasm type to a Rust type
                    #epilog
                }
            }
        };

        token_stream.extend(wrapper_func);
    }

    token_stream
}
