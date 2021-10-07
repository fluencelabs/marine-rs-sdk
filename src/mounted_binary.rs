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

use marine_macro::marine;

use serde::Serialize;
use serde::Deserialize;

pub const SUCCESS_CODE: i32 = 0;

/// Describes result of calling a CLI service.
#[marine]
#[derive(Clone, PartialEq, Default, Eq, Debug, Serialize, Deserialize)]
pub struct MountedBinaryResult {
    /// Return process exit code or host execution error code, where SUCCESS_CODE means success.
    pub ret_code: i32,

    /// Contains the string representation of an error, if ret_code != SUCCESS_CODE.
    pub error: String,

    /// The data that the process wrote to stdout.
    pub stdout: Vec<u8>,

    /// The data that the process wrote to stderr.
    pub stderr: Vec<u8>,
}

/// The same as the MountedBinaryResult, but stdout and stderr are utf8 strings.
#[marine]
#[derive(Clone, PartialEq, Default, Eq, Debug, Serialize, Deserialize)]
pub struct MountedBinaryStringResult {
    /// Return process exit code or host execution error code, where SUCCESS_CODE means success.
    pub ret_code: i32,

    /// Contains the string representation of an error, if ret_code != SUCCESS_CODE.
    pub error: String,

    /// The data that the process wrote to stdout.
    pub stdout: String,

    /// The data that the process wrote to stderr.
    pub stderr: String,
}

impl MountedBinaryResult {
    /// Create a new failure MountedBinaryResult from the provided ret_code.
    pub fn from_error(ret_code: i32, error: impl Into<String>) -> Self {
        Self {
            ret_code,
            error: error.into(),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }

    /// Return true, if this Result represents a success result, otherwise false.
    pub fn is_success(&self) -> bool {
        self.ret_code == SUCCESS_CODE
    }

    /// This function tries to transform a result to the string representation.
    /// Internally, It checks ret_code and returns either Some(Ok(stdout)) if it was SUCCESS_CODE
    /// or Some(Err(error)) otherwise. None is returned if stdout or stderr contains non valid
    /// UTF8 string.
    pub fn into_std(self) -> Option<std::result::Result<String, String>> {
        if self.ret_code == SUCCESS_CODE {
            let stdout = String::from_utf8(self.stdout).ok()?;
            Some(Ok(stdout))
        } else {
            let stderr = String::from_utf8(self.stderr).ok()?;
            Some(Err(format!("error: {}, stderr: {}", self.error, stderr)))
        }
    }

    /// This function tries to represent a result as a string.
    /// Internally, It checks ret_code and returns either Some(Ok(stdout)) if it was SUCCESS_CODE
    /// or Some(Err(error)) otherwise. None is returned if stdout or stderr contains non valid
    /// UTF8 string.
    pub fn as_std(&self) -> Option<std::result::Result<String, String>> {
        if self.ret_code == SUCCESS_CODE {
            let stdout = String::from_utf8(self.stdout.clone()).ok()?;
            Some(Ok(stdout))
        } else {
            let stderr = String::from_utf8(self.stderr.clone()).ok()?;
            Some(Err(format!("error: {}, stderr: {}", self.error, stderr)))
        }
    }

    pub fn stringify(&self) -> Option<MountedBinaryStringResult> {
        let stdout = String::from_utf8(self.stdout.clone()).ok()?;
        let stderr = String::from_utf8(self.stderr.clone()).ok()?;

        let string_result = MountedBinaryStringResult {
            ret_code: self.ret_code,
            error: self.error.clone(),
            stdout,
            stderr,
        };

        Some(string_result)
    }
}
