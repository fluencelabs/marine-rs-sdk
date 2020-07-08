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

use super::GENERATED_FUNC_PREFIX;
use super::GENERATED_SECTION_PREFIX;
use super::GENERATED_GLOBAL_PREFIX;
use crate::fce_ast_types;
use crate::new_ident;

use proc_macro2::TokenStream;
use quote::quote;
use crate::parsed_type::*;

impl quote::ToTokens for fce_ast_types::AstExternModItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // TODO: change serialization protocol
        let fce_type = fce_ast_types::FCEAst::ExternMod(self.clone());
        let data = serde_json::to_vec(&fce_type).unwrap();
        let data_size = data.len();
        let data = syn::LitByteStr::new(&data, proc_macro2::Span::call_site());

        let global_static_name =
            new_ident!(GENERATED_GLOBAL_PREFIX.to_string() + &self.namespace.replace(".", "_"));
        let section_name = GENERATED_SECTION_PREFIX.to_string() + &self.namespace.replace(".", "_");

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

        tokens.extend(glue_code);
    }
}

fn generate_extern_section_items(extern_item: &fce_ast_types::AstExternModItem) -> TokenStream {
    let mut token_stream = TokenStream::new();

    for import in &extern_item.imports {
        let signature = &import.signature;

        let FnEpilogDescriptor { fn_return_type, .. } = signature.output_type.generate_fn_epilog();

        let link_name = import.link_name.as_ref().unwrap_or_else(|| &signature.name);
        let import_name = generate_import_name(&signature.name);
        let ExternDescriptor {
            raw_arg_names,
            raw_arg_types,
        } = signature.input_types.generate_extern_prolog();

        let func = quote! {
            #[link_name = #link_name]
            pub fn #import_name(#(#raw_arg_names: #raw_arg_types),*) #fn_return_type;
        };

        token_stream.extend(func);
    }

    token_stream
}

fn generate_import_name(import_name: &str) -> syn::Ident {
    crate::new_ident!(format!("{}_{}", GENERATED_FUNC_PREFIX, import_name))
}

fn generate_wrapper_functions(extern_item: &fce_ast_types::AstExternModItem) -> TokenStream {
    let mut token_stream = TokenStream::new();

    for import in &extern_item.imports {
        let signature = &import.signature;

        let visibility = new_ident!("pub");
        let func_name = new_ident!(&signature.name);

        let return_type = signature.output_type.generate_wrapper_return_type();
        let import_func_name = generate_import_name(&signature.name);

        let WrapperDescriptor {
            arg_names,
            arg_types,
            raw_args,
        } = signature.input_types.generate_wrapper_prolog();

        let FnEpilogDescriptor {
            return_expression, ..
        } = signature.output_type.generate_fn_epilog();

        let epilog = signature.output_type.generate_wrapper_epilog();

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
