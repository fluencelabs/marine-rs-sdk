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

use crate::ast_types;
use crate::new_ident;
use crate::parsed_type::*;

use proc_macro2::TokenStream;
use quote::quote;

impl quote::ToTokens for ast_types::AstExternMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        crate::prepare_global_data!(
            ExternMod,
            self,
            self.namespace,
            data,
            data_size,
            global_static_name,
            section_name
        );

        let wasm_import_module_name = &self.namespace;
        let generated_imports = generate_extern_section_items(&self);
        let generated_imports_native = generate_extern_section_items_native(&self);
        let wrapper_functions = generate_wrapper_functions(&self);

        let glue_code = quote! {
            #[link(wasm_import_module = #wasm_import_module_name)]
            #[cfg(target_arch = "wasm32")]
            extern "C" {
                #(#generated_imports)*
            }

            #[cfg(not(target_arch = "wasm32"))]
            extern "C" {
                #(#generated_imports_native)*
            }

            #wrapper_functions

            #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #[link_section = #section_name]
            pub static #global_static_name: [u8; #data_size] = { *#data };
        };

        tokens.extend(glue_code);
    }
}

fn generate_extern_section_items(extern_item: &ast_types::AstExternMod) -> Vec<TokenStream> {
    let mut section_items = Vec::with_capacity(extern_item.imports.len());

    for import in &extern_item.imports {
        let signature = &import.signature;
        let fn_return_type = crate::parsed_type::generate_fn_return_type(&signature.output_type);
        let link_name = import.link_name.as_ref().unwrap_or(&signature.name);
        let import_name = generate_import_name(&signature.name);
        let ExternDescriptor {
            raw_arg_names,
            raw_arg_types,
        } = signature.arguments.generate_extern_prolog();

        let func = quote! {
            #[link_name = #link_name]
            fn #import_name(#(#raw_arg_names: #raw_arg_types),*) #fn_return_type;
        };

        section_items.push(func);
    }

    section_items
}

fn generate_extern_section_items_native(extern_item: &ast_types::AstExternMod) -> Vec<TokenStream> {
    let mut section_items = Vec::with_capacity(extern_item.imports.len());

    for import in &extern_item.imports {
        let signature = &import.signature;
        let fn_return_type = match &signature.output_type {
            Some(ty) => quote! {-> #ty},
            None => <_>::default(),
        };

        let link_name = import.link_name.as_ref().unwrap_or(&signature.name);
        let import_name = generate_import_name(&signature.name);

        let unchanged_args: Vec<proc_macro2::TokenStream> = signature
            .arguments
            .iter()
            .map(|arg| {
                let name = new_ident!(&arg.name);
                let ty = &arg.ty;
                quote! {#name : #ty,}
            })
            .collect();

        let func = quote! {
            #[link_name = #link_name]
            fn #import_name(#(#unchanged_args),*) #fn_return_type;
        };

        section_items.push(func);
    }

    section_items
}

#[rustfmt::skip]
fn generate_import_name(import_name: &str) -> syn::Ident {
    crate::new_ident!(format!("{}_{}", super::GENERATED_WRAPPER_FUNC_PREFIX, import_name))
}

fn generate_wrapper_functions(extern_item: &ast_types::AstExternMod) -> TokenStream {
    let mut token_stream = TokenStream::new();

    for import in &extern_item.imports {
        let signature = &import.signature;

        let visibility = &signature.visibility;
        let func_name = new_ident!(&signature.name);

        let return_type = signature.output_type.generate_wrapper_return_type();
        let import_func_name = generate_import_name(&signature.name);

        let WrapperDescriptor {
            arg_names,
            arg_types,
            raw_args,
            arg_transforms,
            arg_drops,
        } = signature.arguments.generate_wrapper_prolog();

        let return_expression =
            crate::parsed_type::generate_return_expression(&signature.output_type);
        let epilog = signature.output_type.generate_wrapper_epilog();

        let wrapper_func = quote! {
            #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #visibility fn #func_name(#(#arg_names: #arg_types), *) #return_type {
                unsafe {

                // make complex arguments manually droppable
                #arg_transforms

                // calling the original function with converted args
                #return_expression #import_func_name(#(#raw_args), *);

                // drop complex arguments
                #arg_drops

                // return value conversation from Wasm type to a Rust type
                #epilog

                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #visibility fn #func_name(#(#arg_names: #arg_types), *) #return_type {
                unsafe {
                // calling the original function with original args
                #import_func_name(#(#arg_names), *)
                }
            }

        };

        token_stream.extend(wrapper_func);
    }

    token_stream
}
