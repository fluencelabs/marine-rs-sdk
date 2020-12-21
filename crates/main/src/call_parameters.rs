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

use serde::Serialize;
use serde::Deserialize;

/// Describes an origin that set an argument.
#[fce]
#[derive(Clone, PartialEq, Default, Eq, Debug, Serialize, Deserialize)]
pub struct SecurityTetraplet {
    pub peer_pk: String,
    pub service_id: String,
    pub function_name: String,
    pub json_path: String,
}

/// This struct contains parameters that would be accessible by Wasm modules.
#[fce]
#[derive(Clone, PartialEq, Default, Eq, Debug, Serialize, Deserialize)]
pub struct CallParameters {
    pub call_id: String,
    pub user_name: String,
    pub application_id: String,
    pub tetraplets: Vec<Vec<SecurityTetraplet>>,
}

impl CallParameters {
    pub fn new<C, U, A>(
        call_id: C,
        user_name: U,
        application_id: A,
        tetraplets: Vec<Vec<SecurityTetraplet>>,
    ) -> Self
    where
        C: Into<String>,
        U: Into<String>,
        A: Into<String>,
    {
        Self {
            call_id: call_id.into(),
            user_name: user_name.into(),
            application_id: application_id.into(),
            tetraplets,
        }
    }
}

/// This functions takes from host current call parameters.
/// Beware that this implies import function call which takes some time.
#[cfg(target_arch = "wasm32")]
pub fn get_call_parameters() -> CallParameters {
    // it's safe until it is executed on standard Fluence node with appropriate import function
    unsafe {
        get_call_raw_parameters();
        let raw_call_parameters = crate::result::get_result_ptr();
        CallParameters::__fce_generated_deserialize(raw_call_parameters as _)
    }
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "host")]
#[allow(improper_ctypes)]
extern "C" {
    // returns serialized current call parameters
    #[link_name = "get_call_parameters"]
    fn get_call_raw_parameters();
}
