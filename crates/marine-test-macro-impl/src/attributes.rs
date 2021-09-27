/*
 * Copyright 2020 Fluence Labs Limited
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

use darling::FromMeta;
use std::collections::HashMap;

/// Describes attributes of `marine_test` macro.
#[derive(Debug, Clone)]
pub(crate) enum MTestAttributes {
    Modules(ModulesAttributes),
    Services(Vec<ServiceDescription>),
}

#[derive(Debug, Default, Clone, FromMeta)]
pub(crate) struct ModulesAttributes {
    /// Path to a config file of a tested service.
    pub(crate) config_path: String,

    /// Path to compiled modules of a service.
    #[darling(default)]
    pub(crate) modules_dir: Option<String>,
}

#[derive(Debug, Default, Clone, FromMeta)]
pub(crate) struct ServiceDescription {
    /// Path to a config file of a tested service.
    pub(crate) config_path: String,

    /// Path to compiled modules of a service.
    #[darling(default)]
    pub(crate) modules_dir: Option<String>,

    #[darling(skip)]
    pub(crate) name: String,
}

impl FromMeta for MTestAttributes {
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        let modules = ModulesAttributes::from_list(items);
        let services = HashMap::<syn::Path, ServiceDescription>::from_list(items);
        match (modules, services) {
            (Ok(modules), Err(_)) => Ok(Self::Modules(modules)),
            (Err(_), Ok(services)) => Ok(Self::Services(process_services(services))),
            (Err(e1), Err(e2)) => Err(darling::error::Error::multiple(vec![e1, e2])),
            (Ok(_), Ok(_)) => Err(darling::Error::custom(
                "internal sdk error: marine_test arguments are ambiguous",
            )),
        }
    }
}

fn process_services(services: HashMap<syn::Path, ServiceDescription>) -> Vec<ServiceDescription> {
    services
        .into_iter()
        .map(|(path, service_desc)| {
            //TODO check if unwrap is safe
            let name = path.segments.last().unwrap().ident.to_string();
            ServiceDescription {
                modules_dir: service_desc.modules_dir,
                config_path: service_desc.config_path,
                name,
            }
        })
        .collect()
}
