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

pub const BINARY_SUCCESS_CODE: i32 = 0;

/// Describes the result of calling a CLI service.
#[derive(Clone, PartialEq, Default, Eq, Debug, Serialize, Deserialize)]
pub struct MountedBinaryResult {
    /// Return process exit code or host execution error code, where SUCCESS_CODE means success.
    pub ret_code: i32,

    /// The data that the process wrote to stdout.
    pub stdout: Vec<u8>,

    /// The data that the process wrote to stderr.
    pub stderr: Vec<u8>,
}

/// Describes the error of calling a CLI service.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MountedBinaryError {
    KilledBySignal,
    LaunchError(String),
}

/// The same as the Result, but stdout and stderr are utf8 strings.
#[derive(Clone, PartialEq, Default, Eq, Debug, Serialize, Deserialize)]
pub struct MountedBinaryStringResult {
    /// Return process exit code or host execution error code, where SUCCESS_CODE means success.
    pub ret_code: i32,

    /// The data that the process wrote to stdout.
    pub stdout: String,

    /// The data that the process wrote to stderr.
    pub stderr: String,
}

impl MountedBinaryResult {
    /// Create a new MountedBinaryResult from the provided ret_code.
    pub fn new(ret_code: i32, stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
        Self {
            ret_code,
            stdout,
            stderr,
        }
    }

    /// Return true, if this Result represents a success result, otherwise false.
    pub fn is_success(&self) -> bool {
        self.ret_code == BINARY_SUCCESS_CODE
    }

    /// This function tries to transform a result to the string representation.
    /// Internally, It checks ret_code and returns either Some(Ok(stdout)) if it was SUCCESS_CODE
    /// or Some(Err(error)) otherwise. None is returned if stdout or stderr contains non valid
    /// UTF8 string.
    pub fn into_std(self) -> Option<std::result::Result<String, String>> {
        if self.ret_code == BINARY_SUCCESS_CODE {
            let stdout = String::from_utf8(self.stdout).ok()?;
            Some(Ok(stdout))
        } else {
            let stderr = String::from_utf8(self.stderr).ok()?;
            Some(Ok(stderr))
        }
    }

    /// This function tries to represent a result as a string.
    /// Internally, It checks ret_code and returns either Some(Ok(stdout)) if it was SUCCESS_CODE
    /// or Some(Err(error)) otherwise. None is returned if stdout or stderr contains non valid
    /// UTF8 string.
    pub fn as_std(&self) -> Option<std::result::Result<String, String>> {
        self.clone().into_std()
    }

    pub fn stringify(&self) -> Option<StringResult> {
        let stdout = String::from_utf8(self.stdout.clone()).ok()?;
        let stderr = String::from_utf8(self.stderr.clone()).ok()?;

        let string_result = StringResult {
            ret_code: self.ret_code,
            stdout,
            stderr,
        };

        Some(string_result)
    }
}

/// This structure is intended for internal usage only. It passed from FCE to sdk and then converts
/// to handy Result<MountedBinaryResult, MountedBinaryError>.
pub struct RawMountedBinaryResult {
    pub ret_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub killed_by_signal: bool,
    pub error: String,
}

impl From<RawMountedBinaryResult> for Result<MountedBinaryResult, MountedBinaryError> {
    fn from(result: RawMountedBinaryResult) -> Self {
        if !result.error.is_empty() {
            return Err(MountedBinaryError::LaunchError(result.error));
        }

        if result.killed_by_signal {
            return Err(MountedBinaryError::KilledBySignal);
        }

        let ok_result = MountedBinaryResult::new(result.ret_code, result.stdout, result.stderr);
        Ok(ok_result)
    }
}
