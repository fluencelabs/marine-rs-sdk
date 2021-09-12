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

use crate::attributes::{MTestAttributes, Services, ServiceDescription};
use crate::TResult;
use crate::TestGeneratorError;
use crate::marine_test;
use crate::marine_test::config_utils;

use fluence_app_service::TomlAppServiceConfig;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use std::path::Path;
use std::path::PathBuf;
use syn::FnArg;
use crate::marine_test::config_utils::Module;
use crate::marine_test::utils::new_ident;

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
    file_path: PathBuf,
) -> TResult<TokenStream> {
    match attrs.services {
        Some(services) => generate_test_glue_code_services(func_item, services, file_path),
        None => generate_test_glue_code_modules(
            func_item,
            attrs.modules_dir,
            attrs.config_path,
            file_path,
        ),
    }
}

fn load_config(
    config_path: &str,
    modules_dir: Option<String>,
    file_path: &PathBuf,
) -> TResult<ConfigWrapper> {
    let config_path_buf = file_path.join(&config_path);

    let marine_config = TomlAppServiceConfig::load(&config_path_buf)?;
    let modules_dir = match config_utils::resolve_modules_dir(&marine_config, modules_dir) {
        Some(modules_dir) => modules_dir,
        None => return Err(TestGeneratorError::ModulesDirUnspecified),
    };

    let modules_dir = file_path.join(modules_dir);
    Ok(ConfigWrapper {
        config: marine_config,
        modules_dir,
    })
}

struct ConfigWrapper {
    config: TomlAppServiceConfig,
    modules_dir: PathBuf,
}

impl ConfigWrapper {
    fn collect_modules(&self) -> TResult<Vec<Module<'_>>> {
        marine_test::config_utils::collect_modules(&self.config, &self.modules_dir)
    }
}

fn generate_services_definitions(
    services: &[ServiceDescription],
    file_path: &PathBuf,
) -> TResult<Vec<TokenStream>> {
    services
        .iter()
        .map(|service| -> TResult<TokenStream> {
            //let service_mod = new_ident(service.name.as_ref().unwrap())?;
            let service_mod = new_ident("test")?;
            let service_definition = generate_service_definition(service, file_path)?;
            let glue_code = quote! {
                pub mod #service_mod {
                    #service_definition
                }
            };

            Ok(glue_code)
        })
        .collect::<TResult<Vec<TokenStream>>>()
}

fn generate_service_definition(
    service: &ServiceDescription,
    file_path: &PathBuf,
) -> TResult<TokenStream> {
    let config_wrapper = load_config(
        &service.config_path,
        service.modules_dir.clone(),
        &file_path,
    )?;
    let module_interfaces = config_wrapper.collect_modules()?;
    let linked_modules = marine_test::modules_linker::link_modules(&module_interfaces)?;
    let modules_dir = match config_utils::resolve_modules_dir(
        &config_wrapper.config,
        service.modules_dir.clone(),
    ) {
        Some(modules_dir) => modules_dir,
        None => return Err(TestGeneratorError::ModulesDirUnspecified),
    };

    let module_definitions = marine_test::module_generator::generate_module_definitions(
        module_interfaces.iter(),
        &linked_modules,
    )?;

    let facade = match module_interfaces.last() {
        Some(module) => module,
        None => return Err(TestGeneratorError::ModulesDirUnspecified),
    };

    let facade_interface = marine_test::module_generator::generate_facade_methods(
        facade.name,
        facade.interface.function_signatures.iter(),
        &facade.interface.record_types,
    )?;
    let app_service_ctor = generate_app_service_ctor(&service.config_path, &modules_dir)?;
    let modules_type = generate_modules_type(&module_interfaces)?;
    let service_definition = quote! {
        pub mod modules {
            #(#module_definitions)*
        }

        #modules_type

        pub struct ServiceInterface {
            pub modules: __GeneratedModules,
            marine: std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>, >
        }

        impl ServiceInterface {
            pub fn new() -> Self {
                #app_service_ctor
                let modules = __GeneratedModules::new(marine.clone());
                Self {
                    marine,
                    modules
                }
            }

            #(#facade_interface)*
        }
    };

    Ok(service_definition)
}

fn generate_modules_type(module_interfaces: &[Module<'_>]) -> TResult<TokenStream> {
    let fields = module_interfaces
        .iter()
        .map(|module| -> TResult<TokenStream> {
            let name = new_ident(module.name)?;
            Ok(quote! {pub #name: modules::#name::ModuleInterface})
        })
        .collect::<TResult<Vec<TokenStream>>>()?;

    let ctors = module_interfaces
        .iter()
        .map(|module| -> TResult<TokenStream> {
            let name = new_ident(module.name)?;
            Ok(quote! {#name: modules::#name::ModuleInterface::new(marine.clone())})
        })
        .collect::<TResult<Vec<TokenStream>>>()?;

    let ty = quote! {
        pub struct __GeneratedModules {
            #(#fields,)*
        }

        impl __GeneratedModules {
            fn new(marine: std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>, >) -> Self {
                Self {
                    #(#ctors,)*
                }
            }
        }
    };

    Ok(ty)
}

fn generate_test_glue_code_modules(
    func_item: syn::ItemFn,
    module_dir: Option<String>,
    config_path: String,
    file_path: PathBuf,
) -> TResult<TokenStream> {
    let config_wrapper = load_config(&config_path, module_dir.clone(), &file_path)?;
    let module_interfaces = config_wrapper.collect_modules()?;
    let linked_modules = marine_test::modules_linker::link_modules(&module_interfaces)?;
    let modules_dir = match config_utils::resolve_modules_dir(&config_wrapper.config, module_dir) {
        Some(modules_dir) => modules_dir,
        None => return Err(TestGeneratorError::ModulesDirUnspecified),
    };

    let module_definitions = marine_test::module_generator::generate_module_definitions(
        module_interfaces.iter(),
        &linked_modules,
    )?;

    let original_block = func_item.block;
    let signature = func_item.sig;
    let name = &signature.ident;
    let inputs = &signature.inputs;
    let arg_names = generate_arg_names(inputs.iter())?;
    let module_ctors = generate_module_ctors(inputs.iter())?;
    let app_service_ctor = generate_app_service_ctor(&config_path, &modules_dir)?;
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

fn generate_test_glue_code_services(
    func_item: syn::ItemFn,
    services: Services,
    file_path: PathBuf,
) -> TResult<TokenStream> {
    let service_definitions = generate_services_definitions(&services.services, &file_path)?;

    let original_block = func_item.block;
    let signature = func_item.sig;
    let name = &signature.ident;
    let glue_code = quote! {
        #[test]
        fn #name() {
            // definitions for wasm modules specified in config
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

fn generate_app_service_ctor(config_path: &str, modules_dir: &Path) -> TResult<TokenStream> {
    let modules_dir = modules_dir
        .to_str()
        .ok_or_else(|| TestGeneratorError::InvalidUTF8Path(modules_dir.to_path_buf()))?;

    let service_ctor = quote! {
        let tmp_dir = std::env::temp_dir();
        let service_id = marine_rs_sdk_test::internal::Uuid::new_v4().to_string();

        let tmp_dir = tmp_dir.join(&service_id);
        let tmp_dir = tmp_dir.to_string_lossy().to_string();
        std::fs::create_dir(&tmp_dir).expect("can't create a directory for service in tmp");

        let mut module_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut file_path = std::path::Path::new(file!()).components();

        let mut truncated_file_path = Vec::new();
        loop {
            if module_path.ends_with(file_path.as_path()) {
                break;
            }

            let (file_path_, remainder) = match file_path.next_back().and_then(|p| match p {
                std::path::Component::Normal(_) | std::path::Component::CurDir | std::path::Component::ParentDir => Some((file_path, p)),
                _ => None,
            }) {
                Some(t) => t,
                None => break,
            };
            file_path = file_path_;

            truncated_file_path.push(remainder);
        }

        for path in truncated_file_path.iter().rev() {
            module_path.push(path);
        }

        let _ = module_path.pop();

        let config_path = module_path.join(#config_path);
        let modules_dir = module_path.join(#modules_dir);
        let modules_dir = modules_dir.to_str().expect("modules_dir contains invalid UTF8 string");

        let mut __m_generated_marine_config = marine_rs_sdk_test::internal::TomlAppServiceConfig::load(&config_path)
            .unwrap_or_else(|e| panic!("app service config located at `{:?}` can't be loaded: {}", config_path, e));
        __m_generated_marine_config.service_base_dir = Some(tmp_dir);
        __m_generated_marine_config.toml_faas_config.modules_dir = Some(modules_dir.to_string());

        let marine = marine_rs_sdk_test::internal::AppService::new_with_empty_facade(__m_generated_marine_config, service_id, std::collections::HashMap::new())
            .unwrap_or_else(|e| panic!("app service can't be created: {}", e));

        let marine = std::rc::Rc::new(std::cell::RefCell::new(marine));
    };

    Ok(service_ctor)
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
