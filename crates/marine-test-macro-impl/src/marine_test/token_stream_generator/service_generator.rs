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
use crate::marine_test::{modules_linker, config_utils};
use crate::marine_test::modules_linker::LinkedModules;
use super::service_generation_utils::generate_service_definition;

use marine_it_parser::it_interface::IModuleInterface;
use proc_macro2::TokenStream;
use itertools::{Itertools, zip};

use std::path::Path;
use std::collections::HashMap;

pub(crate) fn generate_service_definitions(
    services: HashMap<String, ServiceDescription>,
    file_path: &Path,
) -> TResult<Vec<TokenStream>> {
    let services = services
        .into_iter()
        .sorted_by(|lhs, rhs| lhs.0.cmp(&rhs.0)) // deterministic output required for tests
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

pub(super) fn get_facace<'modules, 'm>(
    modules: &'modules [Module<'m>],
) -> TResult<&'modules Module<'m>> {
    match modules.last() {
        Some(module) => Ok(module),
        None => Err(TestGeneratorError::NoModulesInService),
    }
}

pub(super) struct ProcessedService {
    pub(super) config: ConfigWrapper,
    pub(super) config_path: String,
    pub(super) name: String,
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
