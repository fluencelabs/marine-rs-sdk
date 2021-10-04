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

use crate::attributes::{MTestAttributes, ServiceDescription};
use crate::marine_test;
use crate::marine_test::{config_utils, token_stream_generator};
use crate::TestGeneratorError;
use crate::TResult;

use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::FnArg;
use std::collections::HashMap;

/// Generates glue code for tests.
/// F.e. for this test for the greeting service
///```ignore
/// #[marine_test(
///     config_path = "/path/to/service/config/Config.toml",
///     modules_dir = "/path/to/modules/dir"
/// )]
/// fn test() {
///     let result = greeting.greeting("John".to_string());
///     assert_eq(result.as_str(), "Hi, John!");
/// }
/// ```
///
/// the following glue code would be generated:
///```ignore
/// // (0)
///  pub mod __m_generated_greeting {
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
///}
/// // (1)
/// let tmp_dir = std::env::temp_dir();
/// let service_id = marine_rs_sdk_test::internal::Uuid::new_v4().to_string();
///
/// let tmp_dir = tmp_dir.join(&service_id);
/// let tmp_dir = tmp_dir.to_string_lossy().to_string();
/// std::fs::create_dir(&tmp_dir).expect("can't create a directory for service in tmp");
///
/// let mut __m_generated_marine_config = marine_rs_sdk_test::internal::TomlAppServiceConfig::load("/path/to/greeting/Config.toml".to_string())
///     .unwrap_or_else(|e| {
///         panic!(
///              "app service located at `{}` config can't be loaded: {}",
///            "/path/to/greeting/Config.toml", e
///         )
///      });
///
/// __m_generated_marine_config.service_base_dir = Some("/path/to/tmp".to_string());
///
/// let marine = marine_rs_sdk_test::internal::AppService::new_with_empty_facade(
///         __m_generated_marine_config,
///         "3640e972-92e3-47cb-b95f-4e3c5bcf0f14",
///         std::collections::HashMap::new(),
///     ).unwrap_or_else(|e| panic!("app service can't be created: {}", e));
///
/// let marine = std::rc::Rc::new(std::cell::RefCell::new(marine));
///
/// // (2)
///
/// let mut greeting = __m_generated_greeting::MGeneratedStructgreeting::new(marine);
///
/// // (3)
///
/// let result = greeting.greeting("John".to_string());
/// assert_eq(result.as_str(), "Hi, John!");
///
/// // (4)
///```
///
/// Example code above corresponds to the macro definition in the following way:
///      [(0), (1)] - module_definitions*
///      [(1), (2)] - app_service_ctor
///      [(2), (3)] - module_ctors*
///      [(3), (4)] - original_block
pub(super) fn generate_test_glue_code(
    func_item: syn::ItemFn,
    attrs: MTestAttributes,
    test_file_path: PathBuf,
) -> TResult<TokenStream> {
    match attrs {
        MTestAttributes::MultipleServices(services) => {
            generate_test_glue_code_multiple_eservices(func_item, services, test_file_path)
        }
        MTestAttributes::SingleService(service) => {
            generate_test_glue_code_single_service(func_item, service, test_file_path)
        }
    }
}

fn generate_test_glue_code_single_service(
    func_item: syn::ItemFn,
    service: ServiceDescription,
    test_file_path: PathBuf,
) -> TResult<TokenStream> {
    let config_wrapper =
        config_utils::load_config(&service.config_path, service.modules_dir, &test_file_path)?;
    let modules_dir_test_relative = test_file_path.join(&config_wrapper.resolved_modules_dir);
    let module_interfaces =
        config_utils::collect_modules(&config_wrapper.config, &modules_dir_test_relative)?;
    let linked_modules = marine_test::modules_linker::link_modules(
        module_interfaces
            .iter()
            .map(|module| (module.name, &module.interface)),
    )?;

    let module_definitions = token_stream_generator::generate_module_definitions(
        module_interfaces.iter(),
        &linked_modules,
    )?;

    let original_block = func_item.block;
    let signature = func_item.sig;
    let name = &signature.ident;
    let inputs = &signature.inputs;
    let arg_names = generate_arg_names(inputs.iter())?;
    let module_ctors = generate_module_ctors(inputs.iter())?;
    let app_service_ctor = token_stream_generator::generate_app_service_ctor(
        &service.config_path,
        &config_wrapper.resolved_modules_dir,
    )?;
    let glue_code = quote! {
        #[test]
        fn #name() {
            // definitions for wasm modules specified in config
            pub mod marine_test_env {
              #(#module_definitions)*
            }
            // AppService constructor and instantiation to implicit `marine` variable
            #app_service_ctor

            // constructors of all modules of the tested service
            #(#module_ctors)*

            fn test_func(#inputs) {
               #(let mut #arg_names = #arg_names;)*
               // original test function as is
               #original_block
            }

            test_func(#(#arg_names,)*)
        }
    };

    Ok(glue_code)
}

fn generate_test_glue_code_multiple_eservices(
    func_item: syn::ItemFn,
    services: HashMap<String, ServiceDescription>,
    test_file_path: PathBuf,
) -> TResult<TokenStream> {
    let service_definitions =
        token_stream_generator::generate_service_definitions(services, &test_file_path)?;

    let original_block = func_item.block;
    let signature = func_item.sig;
    let name = &signature.ident;
    let glue_code = quote! {
        #[test]
        fn #name() {
            // definitions for services specified in attributes
            pub mod marine_test_env {
              #(#service_definitions)*
            }

            fn test_func() {
               #original_block
            }

            test_func()
        }
    };

    Ok(glue_code)
}

fn generate_module_ctors<'inputs>(
    inputs: impl Iterator<Item = &'inputs FnArg>,
) -> TResult<Vec<TokenStream>> {
    inputs
        .map(|x| -> TResult<_> {
            match x {
                FnArg::Receiver(_) => Err(TestGeneratorError::UnexpectedSelf),
                FnArg::Typed(x) => {
                    let pat = &x.pat;
                    let ty = &x.ty;
                    Ok(quote! {let mut #pat = #ty::new(marine.clone());})
                }
            }
        })
        .collect::<TResult<_>>()
}

fn generate_arg_names<'inputs>(
    inputs: impl Iterator<Item = &'inputs FnArg>,
) -> TResult<Vec<TokenStream>> {
    inputs
        .map(|x| -> TResult<_> {
            match x {
                FnArg::Receiver(_) => Err(TestGeneratorError::UnexpectedSelf),
                FnArg::Typed(x) => Ok(x.pat.to_token_stream()),
            }
        })
        .collect::<TResult<_>>()
}
