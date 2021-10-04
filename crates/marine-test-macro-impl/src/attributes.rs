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
    SingleService(ServiceDescription),
    MultipleServices(HashMap<String, ServiceDescription>),
}

#[derive(Debug, Default, Clone, FromMeta)]
pub(crate) struct ServiceDescription {
    /// Path to a config file of a tested service.
    pub(crate) config_path: String,

    /// Path to compiled modules of a service.
    #[darling(default)]
    pub(crate) modules_dir: Option<String>,
}

impl FromMeta for MTestAttributes {
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        let single_service = ServiceDescription::from_list(items);
        let multiple_services = HashMap::<String, ServiceDescription>::from_list(items);
        match (single_service, multiple_services) {
            (Ok(modules), Err(_)) => Ok(Self::SingleService(modules)),
            (Err(_), Ok(services)) if !services.is_empty() => Ok(Self::MultipleServices(services)),
            (Err(_), Ok(_)) => Err(darling::Error::custom(
                r#"Need to specify "config_path" and "modules_dir" or several named services with these fields "#,
            )),
            (Err(error_single), Err(error_multiple)) => Err(darling::error::Error::multiple(vec![
                error_single,
                error_multiple,
            ])),
            (Ok(_), Ok(_)) => Err(darling::Error::custom(
                "internal sdk error: marine_test attributes are ambiguous",
            )),
        }
    }
}
