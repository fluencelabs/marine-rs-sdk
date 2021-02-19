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

pub const SUCCESS_CODE: i32 = 0;

/// Describes result of calling a CLI service.
#[fce]
#[derive(Clone, PartialEq, Default, Eq, Debug, Serialize, Deserialize)]
pub struct MountedBinaryResult {
    /// Return code, where SUCCESS_CODE means succes.
    pub ret_code: i32,

    /// Contains error message if ret_code != SUCCESS_CODE.
    pub error_message: String,

    /// Contains an output of a CLI service. Note that it could be non-empty even if
    /// there was an error.
    pub result: String,
}
