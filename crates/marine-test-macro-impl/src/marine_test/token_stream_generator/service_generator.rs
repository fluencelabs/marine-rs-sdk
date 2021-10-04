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
use crate::marine_test::config_utils::{Module, ConfigWrapper, load_config};
use crate::marine_test::utils::new_ident;
use crate::marine_test::{modules_linker, config_utils};
use crate::marine_test::modules_linker::{LinkedModules, LinkedModule, UseDescription};

use marine_it_parser::it_interface::IModuleInterface;
use proc_macro2::TokenStream;
use quote::quote;
use itertools::{Itertools, zip};

use std::path::Path;
use std::collections::HashMap;

pub(crate) fn generate_service_definitions(
    services: HashMap<String, ServiceDescription>,
    file_path: &Path,
) -> TResult<Vec<TokenStream>> {
    let services = services
        .into_iter()
        .sorted_by(|lhs, rhs| lhs.0.cmp(&rhs.0))
        .map(|(name, service)| ProcessedService::new(service, name, file_path))
        .collect::<TResult<Vec<ProcessedService>>>()?;

    let service_modules = services
        .iter()
        .map(|service| {
            let modules_dir_test_relative = file_path.join(&service.config.resolved_modules_dir);
            let modules =
                config_utils::collect_modules(&service.config.config, &modules_dir_test_relative)?;
            Ok(modules)
        })
        .collect::<TResult<Vec<Vec<Module<'_>>>>>()?;

    let link_info = link_services(zip(&services, &service_modules))?;
    services
        .iter()
        .map(|service| -> TResult<TokenStream> {
            // entry with service.name was added in link_services(...), so unwrap is safe
            generate_service_definition(
                &service,
                file_path,
                link_info.get::<str>(&service.name).unwrap(),
            )
        })
        .collect::<TResult<Vec<TokenStream>>>()
}

pub(crate) struct ProcessedService {
    pub(crate) config: ConfigWrapper,
    pub(crate) config_path: String,
    pub(crate) name: String,
}

impl ProcessedService {
    pub(crate) fn new(
        service: ServiceDescription,
        name: String,
        file_path: &Path,
    ) -> TResult<Self> {
        let config_wrapper = load_config(&service.config_path, service.modules_dir, &file_path)?;

        Ok(Self {
            config: config_wrapper,
            config_path: service.config_path,
            name,
        })
    }
}

fn link_services<'modules>(
    services: impl ExactSizeIterator<
        Item = (&'modules ProcessedService, &'modules Vec<Module<'modules>>),
    >,
) -> TResult<LinkedModules<'modules>> {
    let facade_modules = services
        .map(|(service, modules)| {
            let facade = get_facace(modules)?;
            Ok((service.name.as_str(), &facade.interface))
        })
        .collect::<TResult<Vec<(&str, &IModuleInterface)>>>()?;

    modules_linker::link_modules(facade_modules.iter().map(|tuple| (tuple.0, tuple.1)))
}

fn generate_service_definition(
    service: &ProcessedService,
    test_file_path: &Path,
    linked_facade: &LinkedModule<'_>,
) -> TResult<TokenStream> {
    let modules_dir_test_relative = test_file_path.join(&service.config.resolved_modules_dir);
    let modules =
        config_utils::collect_modules(&service.config.config, &modules_dir_test_relative)?;
    let linked_modules = modules_linker::link_modules(
        modules
            .iter()
            .map(|module| (module.name, &module.interface)),
    )?;

    let service_mod = new_ident(&service.name)?;
    let module_definitions = super::generate_module_definitions(modules.iter(), &linked_modules)?;

    let facade = get_facace(&modules)?;

    let facade_interface = super::methods_generator::generate_facade_methods(
        facade.interface.function_signatures.iter(),
        &facade.interface.record_types,
    )?;

    let facade_override =
        super::generate_module_definition(facade, linked_facade, service_import_generator)?;
    let facade_override_ident = new_ident("__facade_override")?;
    let facade_structs = generate_facade_structs(facade, &facade_override_ident)?;

    let app_service_ctor =
        generate_app_service_ctor(&service.config_path, &service.config.resolved_modules_dir)?;
    let modules_type = generate_modules_type(&modules)?;

    let service_definition = quote! {
        pub mod #service_mod {
            pub mod modules {
                #(#module_definitions)*
            }

            pub mod #facade_override_ident {
                #facade_override
            }

            #(#facade_structs)*

            #modules_type

            pub struct ServiceInterface {
                pub modules: __GeneratedModules,
                __facade: #facade_override_ident::ModuleInterface,
                marine: std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>, >
            }

            impl ServiceInterface {
                pub fn new() -> Self {
                    #app_service_ctor
                    let modules = __GeneratedModules::new(marine.clone());
                    let __facade = #facade_override_ident::ModuleInterface::new(marine.clone());
                    Self {
                        marine,
                        modules,
                        __facade
                    }
                }

                #(#facade_interface)*
            }
        }
    };

    Ok(service_definition)
}

fn get_facace<'modules, 'm>(modules: &'modules [Module<'m>]) -> TResult<&'modules Module<'m>> {
    match modules.last() {
        Some(module) => Ok(module),
        None => Err(TestGeneratorError::NoModulesInService),
    }
}

fn service_import_generator(info: &UseDescription<'_>) -> TResult<TokenStream> {
    let from_module_ident = new_ident(info.from)?;
    let record_name_ident = new_ident(info.name)?;
    Ok(quote! {pub use super::super::#from_module_ident::#record_name_ident;})
}

fn generate_facade_structs(
    module: &Module<'_>,
    module_name: &syn::Ident,
) -> TResult<Vec<TokenStream>> {
    module
        .interface
        .record_types
        .iter()
        .sorted_by_key(|(_, record)| &record.name)
        .map(|(_, record)| -> TResult<TokenStream> {
            let record_name = new_ident(&record.name)?;
            let result = quote! {pub use #module_name::#record_name;};
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
