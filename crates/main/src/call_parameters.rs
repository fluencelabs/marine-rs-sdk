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

use fluence_sdk_macro::fce;

/// This struct contains parameters that would be accessible by Wasm modules.
#[fce]
#[derive(Clone, PartialEq, Default, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct CallParameters {
    pub call_id: String,
    pub user_name: String,
    pub application_id: String,
}

impl CallParameters {
    pub fn new<C, U, A>(call_id: C, user_name: U, application_id: A) -> Self
    where
        C: Into<String>,
        U: Into<String>,
        A: Into<String>,
    {
        Self {
            call_id: call_id.into(),
            user_name: user_name.into(),
            application_id: application_id.into(),
        }
    }
}

#[cfg(target_arch = "wasm")]
#[fce]
#[link(wasm_import_module = "host")]
#[allow(improper_ctypes)]
extern "C" {
    // returns current call parameters
    pub fn get_call_parameters() -> CallParameters;
}
