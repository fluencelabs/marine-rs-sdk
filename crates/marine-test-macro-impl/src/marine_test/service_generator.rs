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
use crate::attributes::{ServiceDescription};
use crate::TResult;
use crate::TestGeneratorError;
use crate::marine_test;
use crate::marine_test::config_utils;

use proc_macro2::TokenStream;
use quote::quote;

use std::path::Path;
use std::path::PathBuf;
use crate::marine_test::config_utils::{Module, ConfigWrapper};
use crate::marine_test::utils::new_ident;

pub(crate) fn generate_services_definitions(
    services: Vec<ServiceDescription>,
    file_path: &PathBuf,
) -> TResult<Vec<TokenStream>> {
    let processed_services = services
        .into_iter()
        .map(|service| -> TResult<ProcessedService> { ProcessedService::new(service, file_path) })
        .collect::<TResult<Vec<ProcessedService>>>()?;

    processed_services
        .iter()
        .map(|service| -> TResult<TokenStream> {
            let service_mod = new_ident(&service.name)?;
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

struct ProcessedService {
    config: ConfigWrapper,
    config_path: String,
    name: String,
}

impl ProcessedService {
    pub fn new(service: ServiceDescription, file_path: &PathBuf) -> TResult<Self> {
        let config_wrapper =
            config_utils::load_config(&service.config_path, service.modules_dir, &file_path)?;

        Ok(Self {
            config: config_wrapper,
            config_path: service.config_path,
            name: service.name,
        })
    }
}

fn generate_service_definition(
    service: &ProcessedService,
    file_path: &PathBuf,
) -> TResult<TokenStream> {
    let modules = service.config.collect_modules(file_path)?;
    let linked_modules = marine_test::modules_linker::link_modules(&modules)?;

    let module_definitions = marine_test::module_generator::generate_module_definitions(
        modules.iter(),
        &linked_modules,
    )?;

    let facade = match modules.last() {
        Some(module) => module,
        None => return Err(TestGeneratorError::ZeroModules),
    };

    let facade_name = new_ident(&facade.name)?;
    let facade_interface = marine_test::module_generator::generate_facade_methods(
        &facade.name,
        facade.interface.function_signatures.iter(),
        &facade.interface.record_types,
    )?;
    let facade_structs = generate_facade_structs(facade, &facade_name)?;

    let app_service_ctor =
        generate_app_service_ctor(&service.config_path, &service.config.modules_dir)?;
    let modules_type = generate_modules_type(&modules)?;

    let service_definition = quote! {
        pub mod modules {
            #(#module_definitions)*
        }

        #(#facade_structs)*

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

fn generate_facade_structs(
    module: &Module<'_>,
    module_name: &syn::Ident,
) -> TResult<Vec<TokenStream>> {
    module
        .interface
        .record_types
        .iter()
        .map(|(_, record)| -> TResult<TokenStream> {
            let record_name = new_ident(&record.name)?;
            let result = quote! {pub use modules::#module_name::#record_name;};
            Ok(result)
        })
        .collect::<TResult<Vec<TokenStream>>>()
}

pub(crate) fn generate_app_service_ctor(
    config_path: &str,
    modules_dir: &Path,
) -> TResult<TokenStream> {
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
