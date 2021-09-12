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
use syn::NestedMeta;

/// Describes attributes of `marine_test` macro.
#[derive(Debug, Default, Clone, FromMeta)]
pub(crate) struct MTestAttributes {
    /// Path to a config file of a tested service.
    pub(crate) config_path: String,

    /// Path to compiled modules of a service.
    #[darling(default)]
    pub(crate) modules_dir: Option<String>,

    #[darling(default)]
    pub(crate) services: Option<Services>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Services {
    pub(crate) services: Vec<ServiceDescription>,
}

#[derive(Debug, Default, Clone, FromMeta)]
pub(crate) struct ServiceDescription {
    /// Path to a config file of a tested service.
    pub(crate) config_path: String,

    /// Path to compiled modules of a service.
    #[darling(default)]
    pub(crate) modules_dir: Option<String>,
}

impl FromMeta for Services {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let services = items
            .iter()
            .map(|item| match item {
                NestedMeta::Meta(meta) => ServiceDescription::from_meta(meta),
                _ => Err(darling::Error::custom("Expected array dfgh").with_span(item)),
            })
            .collect::<darling::Result<Vec<ServiceDescription>>>()?;

        Ok(Services { services })
    }
}
