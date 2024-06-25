/*
 * Fluence Marine Rust SDK
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#[cfg(all(target_arch = "wasm32", feature = "marine-abi"))]
use marine_macro::marine;

use serde::Serialize;
use serde::Deserialize;

pub const SUCCESS_CODE: i32 = 0;

/// Describes result of calling a CLI service.
#[cfg_attr(all(target_arch = "wasm32", feature = "marine-abi"), marine)]
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
#[cfg_attr(all(target_arch = "wasm32", feature = "marine-abi"), marine)]
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
