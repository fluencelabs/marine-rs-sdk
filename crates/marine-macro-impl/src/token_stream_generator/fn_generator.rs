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
use crate::parsed_type::FnEpilogGlueCodeGenerator;
use crate::parsed_type::FnEpilogDescriptor;
use crate::parsed_type::FnEpilogIngredients;
use crate::parsed_type::FnPrologGlueCodeGenerator;
use crate::parsed_type::FnPrologDescriptor;

use crate::new_ident;

use proc_macro2::TokenStream;

impl quote::ToTokens for ast_types::AstFn {
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

                // save objects to keep them in memory for lifting
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
