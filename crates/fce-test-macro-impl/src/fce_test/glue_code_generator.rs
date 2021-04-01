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

use fluence_app_service::TomlAppServiceConfig;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use std::path::PathBuf;

/// Generates glue code for tests.
/// F.e. for the greeting service the following glue code would be generated:
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
///             let arguments = fluence_test::internal::json!([name]);
///             let result = self
///                 .fce
///                 .as_ref
///                 .borrow_mut()
///                 .call_with_module_name("greeting", "greeting", arguments, <_>::default())
///                 .expect("call to FCE failed");
///             let result: String = serde_json::from_value(result)
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
/// let mut __fce__generated_fce_config = fluence_test::internal::TomlAppServiceConfig::load("/path/to/greeting/Config.toml".to_string())
///     .unwrap_or_else(|e| {
///         panic!(
///              "app service located at `{}` config can't be loaded: {}",
///            "/Users/mike/dev/work/fluence/wasm/fce/examples/greeting/Config.toml", e
///         )
///      });
///
/// __fce__generated_fce_config.service_base_dir = Some("/path/to/tmp".to_string());
///
/// let fce = fluence_test::internal::AppService::new_with_empty_facade(
///         __fce__generated_fce_config,
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
///```
///
/// Here [(0), (1)] - module_definitions
///      [(1), (2)] - fce ctor
///      [(2), (3)] - ctors of all modules of the tested service
///      [(3), (4)] - original test function
pub(super) fn generate_test_glue_code(
    func_item: syn::ItemFn,
    attrs: FCETestAttributes,
) -> TResult<TokenStream> {
    let fce_config = TomlAppServiceConfig::load(&attrs.config_path)?;
    let modules_dir =
        match fce_test::config_worker::determine_modules_dir(&fce_config, attrs.modules_dir) {
            Some(modules_dir) => modules_dir,
            None => return Err(TestGeneratorError::ModulesDirUnspecified),
        };

    let fce_ctor = generate_fce_ctor(&attrs.config_path, &modules_dir);
    let module_interfaces = fce_test::config_worker::collect_modules(&fce_config, modules_dir)?;

    let module_definitions =
        fce_test::module_generator::generate_module_definitions(module_interfaces.iter())?;
    let module_iter = module_interfaces.iter().map(|module| module.name);
    let module_ctors = generate_module_ctors(module_iter)?;
    let original_block = func_item.block;
    let signature = func_item.sig;

    let glue_code = quote! {
        #[test]
        #signature {
            #module_definitions

            #fce_ctor

            #module_ctors

            #original_block
        }
    };

    Ok(glue_code)
}

fn generate_fce_ctor(config_path: &str, modules_dir: &PathBuf) -> TokenStream {
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
) -> TResult<TokenStream> {
    let mut module_ctors = Vec::with_capacity(module_names.len());
    for name in module_names {
        // TODO: optimize these two call because they are called twice for each module name
        // and internally allocate memory in format call.
        let module_name = fce_test::utils::generate_module_name(&name)?;
        let struct_name = fce_test::utils::generate_struct_name(&name)?;
        let name_for_user = fce_test::utils::new_ident(&name)?;

        let module_ctor =
            quote! { let mut #name_for_user = #module_name::#struct_name::new(fce.clone()); };

        module_ctors.push(module_ctor);
    }

    let module_ctors = quote! { #(#module_ctors),* };

    Ok(module_ctors)
}
