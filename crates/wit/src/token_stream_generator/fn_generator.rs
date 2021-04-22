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
use crate::parsed_type::FnEpilogGlueCodeGenerator;
use crate::parsed_type::FnEpilogDescriptor;
use crate::parsed_type::FnEpilogIngredients;
use crate::parsed_type::FnPrologGlueCodeGenerator;
use crate::parsed_type::FnPrologDescriptor;

use crate::new_ident;

use proc_macro2::TokenStream;

impl quote::ToTokens for ast_types::AstFnItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        crate::prepare_global_data!(
            Function,
            self,
            self.signature.name,
            data,
            data_size,
            global_static_name,
            section_name
        );

        let signature = &self.signature;
        let func_name = new_ident!(format!(
            "{}{}",
            super::GENERATED_WRAPPER_FUNC_PREFIX,
            signature.name
        ));
        let original_func_ident = new_ident!(signature.name);
        let export_func_name = &signature.name;

        let FnPrologDescriptor {
            raw_arg_names,
            raw_arg_types,
            prolog,
            converted_arg_idents,
            args,
        } = &signature.arguments.generate_prolog();

        let epilog_ingredients = FnEpilogIngredients {
            args: &signature.arguments,
            converted_args: converted_arg_idents,
            return_type: &signature.output_type,
        };

        let FnEpilogDescriptor {
            fn_return_type,
            return_expression,
            epilog,
            objs_savings,
        } = epilog_ingredients.generate_fn_epilog();

        // here this Option must be Some
        let original_func = &self.original;

        let glue_code = quote::quote! {
            #original_func

            #[cfg(target_arch = "wasm32")]
            #[export_name = #export_func_name]
            #[no_mangle]
            #[doc(hidden)]
            #[allow(clippy::all)]
            pub unsafe fn #func_name(#(#raw_arg_names: #raw_arg_types),*) #fn_return_type {
                // arguments conversation from Wasm types to Rust types
                #prolog

                // calling the original function with converted args
                #return_expression #original_func_ident(#(#args), *);

                // return value conversation from Rust type to a Wasm type
                #epilog

                // save objects to keep them in memory that allows IT side
                #objs_savings
            }

            #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #[link_section = #section_name]
            pub static #global_static_name: [u8; #data_size] = { *#data };
        };

        tokens.extend(glue_code);
    }
}
