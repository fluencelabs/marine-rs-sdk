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

#![doc(html_root_url = "https://docs.rs/sdk-test/0.3.0")]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]
#![warn(rust_2018_idioms)]

pub use marine_test_macro::marine_test;
pub use marine_test_macro::fce_test;

pub use fluence_app_service::CallParameters;
pub use fluence_app_service::SecurityTetraplet;

/// These API functions are intended for internal usage in generated code.
/// Normally, you shouldn't use them.
pub mod internal {
    pub use fluence_app_service::AppService;
    pub use fluence_app_service::TomlAppServiceConfig;

    pub use serde;
    pub use serde_json;

    pub use uuid::Uuid;
}
