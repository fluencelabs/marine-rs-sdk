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

#![allow(dead_code)]

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_SIZE: usize = PKG_VERSION.len();

const fn sdk_version() -> [u8; VERSION_SIZE] {
    let version_as_slice = PKG_VERSION.as_bytes();

    let mut version_as_array: [u8; VERSION_SIZE] = [0; VERSION_SIZE];
    let mut byte_id = 0;
    while byte_id < VERSION_SIZE {
        version_as_array[byte_id] = version_as_slice[byte_id];
        byte_id += 1;
    }

    version_as_array
}

// TODO: avoid duplication with the link_section when key-value attributes become stable
pub const VERSION_SECTION_NAME: &str = "__fluence_sdk_version";

#[cfg(target_arch = "wasm32")]
#[link_section = "__fluence_sdk_version"]
#[doc(hidden)]
pub static __M_SDK_VERSION: [u8; VERSION_SIZE] = sdk_version();
