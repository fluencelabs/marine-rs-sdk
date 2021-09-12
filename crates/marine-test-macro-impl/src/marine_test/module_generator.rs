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
mod methods_generator_utils;
mod record_type_generator;

use crate::marine_test::utils;
use crate::marine_test::config_utils::Module;
use crate::TResult;
use crate::marine_test::modules_linker::{LinkedModules, LinkedModule};

use proc_macro2::TokenStream;
use quote::quote;

/// Generates definitions of modules and records of this modules.
/// F.e. for the greeting service the following definitions would be generated:
///```ignore
/// pub mod __m_generated_greeting {
///     struct MGeneratedStructgreeting {
///         marine: std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>>,
///     }
///
///     impl MGeneratedStructgreeting {
///         pub fn new(marine: std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>>) -> Self {
///             Self { marine }
///         }
///
///         pub fn greeting(&mut self, name: String) -> String {
///             use std::ops::DerefMut;
///             let arguments = marine_rs_sdk_test::internal::serde_json::json!([name]);
///             let result = self
///                 .marine
///                 .as_ref
///                 .borrow_mut()
///                 .call_with_module_name("greeting", "greeting", arguments, <_>::default())
///                 .expect("call to Marine failed");
///             let result: String = marine_rs_sdk_test::internal::serde_json::from_value(result)
///                 .expect("the default deserializer shouldn't fail");
///             result
///         }
///     }
/// }
///```
pub(super) fn generate_module_definitions<'i>(
    modules: impl ExactSizeIterator<Item = &'i Module<'i>>,
    linked_modules: &'i LinkedModules<'_>,
) -> TResult<Vec<TokenStream>> {
    modules
        .into_iter()
        .map(|value| generate_module_definition(value, linked_modules.get(&value.name).unwrap())) // linked_modules are built from modules
        .collect::<TResult<Vec<_>>>()
}

pub(super) use methods_generator::generate_facade_methods;

fn generate_module_definition(
    module: &Module<'_>,
    linked_module: &'_ LinkedModule<'_>,
) -> TResult<TokenStream> {
    let module_name = module.name;
    let module_ident = utils::new_ident(module_name)?;
    let struct_ident = utils::new_ident("ModuleInterface")?;

    let module_interface = &module.interface;
    let module_records = record_type_generator::generate_records(linked_module)?;
    let module_functions = methods_generator::generate_module_methods(
        module_name,
        module_interface.function_signatures.iter(),
        &module_interface.record_types,
    )?;

    let module_definition = quote! {
        // it's a sort of hack: this module structure allows user to import structs by
        // using marine_env_test::module_name::StructName;
        pub mod #module_ident {
            #(#module_records)*

            pub struct #struct_ident {
                marine: std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>, >,
            }

            impl #struct_ident {
                pub fn new(marine: std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>, >) -> Self {
                    Self { marine }
                }
            }

            impl #struct_ident {
                #(#module_functions)*
            }
        }
    };

    Ok(module_definition)
}
