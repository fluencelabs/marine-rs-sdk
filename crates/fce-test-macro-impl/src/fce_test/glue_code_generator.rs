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

use crate::attributes::FCETestAttributes;
use crate::TResult;
use crate::TestGeneratorError;
use crate::fce_test;
use crate::fce_test::config_utils;

use fluence_app_service::TomlAppServiceConfig;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use std::path::Path;

/// Generates glue code for tests.
/// F.e. for this test for the greeting service
///```ignore
/// #[fce_test(
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
///  pub mod __fce_generated_greeting {
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
///}
/// // (1)
/// let tmp_dir = std::env::temp_dir();
/// let service_id = fluence_test::internal::Uuid::new_v4().to_string();
///
/// let tmp_dir = tmp_dir.join(&service_id);
/// let tmp_dir = tmp_dir.to_string_lossy().to_string();
/// std::fs::create_dir(&tmp_dir).expect("can't create a directory for service in tmp");
///
/// let mut __fce_generated_fce_config = fluence_test::internal::TomlAppServiceConfig::load("/path/to/greeting/Config.toml".to_string())
///     .unwrap_or_else(|e| {
///         panic!(
///              "app service located at `{}` config can't be loaded: {}",
///            "/path/to/greeting/Config.toml", e
///         )
///      });
///
/// __fce_generated_fce_config.service_base_dir = Some("/path/to/tmp".to_string());
///
/// let fce = fluence_test::internal::AppService::new_with_empty_facade(
///         __fce_generated_fce_config,
///         "3640e972-92e3-47cb-b95f-4e3c5bcf0f14",
///         std::collections::HashMap::new(),
///     ).unwrap_or_else(|e| panic!("app service can't be created: {}", e));
///
/// let fce = std::rc::Rc::new(std::cell::RefCell::new(fce));
///
/// // (2)
///
/// let mut greeting = __fce_generated_greeting::FCEGeneratedStructgreeting::new(fce);
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
    attrs: FCETestAttributes,
) -> TResult<TokenStream> {
    let fce_config = TomlAppServiceConfig::load(&attrs.config_path)?;
    let modules_dir = match config_utils::resolve_modules_dir(&fce_config, attrs.modules_dir) {
        Some(modules_dir) => modules_dir,
        None => return Err(TestGeneratorError::ModulesDirUnspecified),
    };

    let app_service_ctor = generate_app_service_ctor(&attrs.config_path, &modules_dir);
    let module_interfaces = fce_test::config_utils::collect_modules(&fce_config, modules_dir)?;

    let module_definitions =
        fce_test::module_generator::generate_module_definitions(module_interfaces.iter())?;

    let module_iter = module_interfaces.iter().map(|module| module.name);
    let module_ctors = generate_module_ctors(module_iter)?;

    let original_block = func_item.block;
    let signature = func_item.sig;

    let glue_code = quote! {
        #[test]
        #signature {
            // definitions for wasm modules specified in config
            #(#module_definitions)*

            // AppService constructor and instantiation to implicit `fce` variable
            #app_service_ctor

            // constructors of all modules of the tested service
            #(#module_ctors)*

            // original test function as is
            #original_block
        }
    };

    Ok(glue_code)
}

fn generate_app_service_ctor(config_path: &str, modules_dir: &Path) -> TokenStream {
    let config_path = config_path.to_token_stream();
    let modules_dir = modules_dir.to_string_lossy().to_string();

    quote! {
        let tmp_dir = std::env::temp_dir();
        let service_id = fluence_test::internal::Uuid::new_v4().to_string();

        let tmp_dir = tmp_dir.join(&service_id);
        let tmp_dir = tmp_dir.to_string_lossy().to_string();
        std::fs::create_dir(&tmp_dir).expect("can't create a directory for service in tmp");

        let mut __fce_generated_fce_config = fluence_test::internal::TomlAppServiceConfig::load(#config_path.to_string())
            .unwrap_or_else(|e| panic!("app service located at `{}` config can't be loaded: {}", #config_path, e));
        __fce_generated_fce_config.service_base_dir = Some(tmp_dir);
        __fce_generated_fce_config.toml_faas_config.modules_dir = Some(#modules_dir.to_string());

        let fce = fluence_test::internal::AppService::new_with_empty_facade(__fce_generated_fce_config, service_id, std::collections::HashMap::new())
            .unwrap_or_else(|e| panic!("app service can't be created: {}", e));

        let fce = std::rc::Rc::new(std::cell::RefCell::new(fce));
    }
}

fn generate_module_ctors<'n>(
    module_names: impl ExactSizeIterator<Item = &'n str>,
) -> TResult<Vec<TokenStream>> {
    module_names
        .map(|name| -> TResult<_> {
            // TODO: optimize these two call because they are called twice for each module name
            // and internally allocate memory in format call.
            let module_name = fce_test::utils::generate_module_name(&name)?;
            let struct_name = fce_test::utils::generate_struct_name(&name)?;
            let name_for_user = fce_test::utils::new_ident(&name)?;

            let module_ctor =
                quote! { let mut #name_for_user = #module_name::#struct_name::new(fce.clone()); };

            Ok(module_ctor)
        })
        .collect::<TResult<_>>()
}
