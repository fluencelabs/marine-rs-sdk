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

#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
#[link_section = "__fluence_sdk_version"]
#[doc(hidden)]
pub static __M_SDK_VERSION: [u8; VERSION_SIZE] = sdk_version();
