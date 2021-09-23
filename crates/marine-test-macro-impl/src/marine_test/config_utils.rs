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

use crate::{TResult, TestGeneratorError};

use fluence_app_service::TomlAppServiceConfig;
use marine_it_parser::module_it_interface;
use marine_it_parser::it_interface::IModuleInterface;

use std::path::PathBuf;
use crate::attributes::ServiceDescription;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Module<'m> {
    pub name: &'m str,
    pub interface: IModuleInterface,
}

impl<'m> Module<'m> {
    fn new(name: &'m str, interface: IModuleInterface) -> Self {
        Self { name, interface }
    }
}

pub(crate) struct ConfigWrapper {
    pub config: TomlAppServiceConfig,
    pub modules_dir: PathBuf,
}

impl ConfigWrapper {
    pub(super) fn collect_modules(&self, file_path: &PathBuf) -> TResult<Vec<Module<'_>>> {
        let modules_dir = file_path.join(&self.modules_dir);
        collect_modules(&self.config, &modules_dir)
    }
}

pub(crate) struct ProcessedService {
    pub(crate) config: ConfigWrapper,
    pub(crate) config_path: String,
    pub(crate) name: String,
}

impl ProcessedService {
    pub(crate) fn new(service: ServiceDescription, file_path: &PathBuf) -> TResult<Self> {
        let config_wrapper =
            load_config(&service.config_path, service.modules_dir, &file_path)?;

        Ok(Self {
            config: config_wrapper,
            config_path: service.config_path,
            name: service.name,
        })
    }
}

pub(crate) fn load_config(
    config_path: &str,
    modules_dir: Option<String>,
    file_path: &PathBuf,
) -> TResult<ConfigWrapper> {
    let config_path_buf = file_path.join(&config_path);

    let marine_config = TomlAppServiceConfig::load(&config_path_buf)?;
    let modules_dir = match resolve_modules_dir(&marine_config, modules_dir) {
        Some(modules_dir) => modules_dir,
        None => return Err(TestGeneratorError::ModulesDirUnspecified),
    };

    Ok(ConfigWrapper {
        config: marine_config,
        modules_dir,
    })
}

/// Returns all modules the provided config consists of.
pub(super) fn collect_modules<'config>(
    config: &'config TomlAppServiceConfig,
    modules_dir: &PathBuf,
) -> TResult<Vec<Module<'config>>> {
    let module_paths = collect_module_paths(config, &modules_dir);

    module_paths
        .into_iter()
        .map(|(name, path)| module_it_interface(path).map(|interface| Module::new(name, interface)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn collect_module_paths<'config>(
    config: &'config TomlAppServiceConfig,
    modules_dir: &PathBuf,
) -> Vec<(&'config str, PathBuf)> {
    config
        .toml_faas_config
        .module
        .iter()
        .map(|m| {
            let module_file_name = m.file_name.as_ref().unwrap_or(&m.name);
            let module_file_name = PathBuf::from(module_file_name);
            // TODO: is it correct to always have .wasm extension?
            let module_path = modules_dir.join(module_file_name).with_extension("wasm");

            (m.name.as_str(), module_path)
        })
        .collect::<Vec<_>>()
}

/// Tries to determine a dir with compiled Wasm modules according to the following rules:
///  - if the modules_dir attribute is specified (by user) it will be chosen,
///  - otherwise if modules_dir is specified in AppService config it will be chosen,
///  - otherwise None will be returned.
pub(super) fn resolve_modules_dir(
    config: &TomlAppServiceConfig,
    modules_dir: Option<String>,
) -> Option<PathBuf> {
    match modules_dir {
        Some(modules_dir) => Some(PathBuf::from(modules_dir)),
        None => config
            .toml_faas_config
            .modules_dir
            .as_ref()
            .map(PathBuf::from),
    }
}
