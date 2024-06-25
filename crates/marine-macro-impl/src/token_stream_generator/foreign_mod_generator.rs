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
        let wasm_section_items = generate_extern_section_items(self, wasm_extern_item_generator);
        let not_wasm_section_items =
            generate_extern_section_items(self, not_wasm_extern_item_generator);

        let wrapper_functions = generate_wrapper_functions(self);

        let glue_code = quote! {
            #[link(wasm_import_module = #wasm_import_module_name)]
            #[cfg(target_arch = "wasm32")]
            extern "C" {
                #(#wasm_section_items)*
            }

            #[cfg(not(target_arch = "wasm32"))]
            extern "C" {
                #(#not_wasm_section_items)*
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

fn generate_extern_section_items(
    extern_item: &ast_types::AstExternMod,
    item_generator: fn(&ast_types::AstExternFn) -> TokenStream,
) -> Vec<TokenStream> {
    let mut section_items = Vec::with_capacity(extern_item.imports.len());

    for import in &extern_item.imports {
        section_items.push(item_generator(import));
    }

    section_items
}

fn wasm_extern_item_generator(import: &ast_types::AstExternFn) -> TokenStream {
    let signature = &import.signature;
    let fn_return_type = crate::parsed_type::generate_fn_return_type(&signature.output_type);
    let link_name = import.link_name.as_ref().unwrap_or(&signature.name);
    let import_name = generate_import_name(&signature.name);
    let ExternDescriptor {
        raw_arg_names,
        raw_arg_types,
    } = signature.arguments.generate_extern_prolog();

    quote! {
        #[link_name = #link_name]
        fn #import_name(#(#raw_arg_names: #raw_arg_types),*) #fn_return_type;
    }
}

fn not_wasm_extern_item_generator(import: &ast_types::AstExternFn) -> TokenStream {
    let signature = &import.signature;
    let original_return_type =
        crate::parsed_type::generate_fn_original_return_type(&signature.output_type);
    let link_name = import.link_name.as_ref().unwrap_or(&signature.name);
    let import_name = generate_import_name(&signature.name);
    let original_arguments = generate_original_arguments(&signature.arguments);

    quote! {
        #[link_name = #link_name]
        fn #import_name(#(#original_arguments),*) #original_return_type;
    }
}

fn generate_original_arguments(
    arguments: &[ast_types::AstFnArgument],
) -> Vec<proc_macro2::TokenStream> {
    arguments
        .iter()
        .map(|arg| {
            let name = crate::new_ident!(&arg.name);
            let ty = &arg.ty;
            quote! {#name : #ty}
        })
        .collect()
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
