/*
 * Copyright 2021 Fluence Labs Limited
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

mod methods_generator;
mod record_type_generator;

use crate::fce_test::utils;
use crate::fce_test::config_utils::Module;
use crate::TResult;

use proc_macro2::TokenStream;
use quote::quote;

/// Generates definitions of modules and records of this modules.
/// F.e. for the greeting service the following definitions would be generated:
///```ignore
/// pub mod __fce_generated_greeting {
///     struct FCEGeneratedStructgreeting {
///         fce: std::rc::Rc<std::cell::RefCell<fluence_test::internal::AppService>>,
///     }
///
///     impl FCEGeneratedStructgreeting {
///         pub fn new(fce: std::rc::Rc<std::cell::RefCell<fluence_test::internal::AppService>>) -> Self {
///             Self { fce }
///         }
///
///         pub fn greeting(&mut self, name: String) -> String {
///             use std::ops::DerefMut;
///             let arguments = fluence_test::internal::serde_json::json!([name]);
///             let result = self
///                 .fce
///                 .as_ref
///                 .borrow_mut()
///                 .call_with_module_name("greeting", "greeting", arguments, <_>::default())
///                 .expect("call to FCE failed");
///             let result: String = fluence_test::internal::serde_json::from_value(result)
///                 .expect("the default deserializer shouldn't fail");
///             result
///         }
///     }
/// }
///```
pub(super) fn generate_module_definitions<'i>(
    modules: impl ExactSizeIterator<Item = &'i Module<'i>>,
) -> TResult<Vec<TokenStream>> {
    modules
        .into_iter()
        .map(generate_module_definition)
        .collect::<TResult<Vec<_>>>()
}

fn generate_module_definition(module: &Module<'_>) -> TResult<TokenStream> {
    let module_name = module.name;
    let module_name_ident = utils::generate_module_name(module_name)?;
    let struct_name_ident = utils::generate_struct_name(module_name)?;

    let module_interface = &module.interface;
    let module_records = record_type_generator::generate_records(&module_interface.record_types)?;
    let module_functions = methods_generator::generate_module_methods(
        module_name,
        module_interface.function_signatures.iter(),
        &module_interface.record_types,
    )?;

    let module_definition = quote! {
        pub mod #module_name_ident {
            #(#module_records)*

            pub struct #struct_name_ident {
                fce: std::rc::Rc<std::cell::RefCell<fluence_test::internal::AppService>>,
            }

            impl #struct_name_ident {
                pub fn new(fce: std::rc::Rc<std::cell::RefCell<fluence_test::internal::AppService>>) -> Self {
                    Self { fce }
                }
            }

            impl #struct_name_ident {
                #(#module_functions)*
            }
        }
    };

    Ok(module_definition)
}
