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

// TODO: avoid duplication with the link_section when key-value attributes become stable
pub const MANIFEST_SECTION_NAME: &str = "__fluence_wasm_module_manifest";

#[macro_export]
macro_rules! module_manifest {
    ($authors:expr, $version:expr, $description:expr, $repository:expr) => {
        fce_timestamp_macro::build_timestamp!();

        const __FCE_SDK_AUTHORS_SIZE: usize = $authors.as_bytes().len();
        const __FCE_SDK_VERSION_SIZE: usize = $version.as_bytes().len();
        const __FCE_SDK_DESCRIPTION_SIZE: usize = $description.as_bytes().len();
        const __FCE_SDK_REPOSITORY_SIZE: usize = $repository.as_bytes().len();
        const __FCE_SDK_BUILD_TIME_SIZE: usize = __FCE_SDK_BUILD_TIME.as_bytes().len();
        const __FCE_SDK_FIELD_PREFIX_SIZE: usize = std::mem::size_of::<u64>();

        const __FCE_MANIFEST_SIZE: usize = __FCE_SDK_AUTHORS_SIZE
            + __FCE_SDK_VERSION_SIZE
            + __FCE_SDK_DESCRIPTION_SIZE
            + __FCE_SDK_REPOSITORY_SIZE
            + __FCE_SDK_BUILD_TIME_SIZE
            + __FCE_SDK_FIELD_PREFIX_SIZE * 5;

        const fn __fce_sdk_append_data(
            mut manifest: [u8; __FCE_MANIFEST_SIZE],
            data: &'static str,
            offset: usize,
        ) -> ([u8; __FCE_MANIFEST_SIZE], usize) {
            let data_as_bytes = data.as_bytes();
            let data_len = data_as_bytes.len();

            // write data prefix with data size in LE
            let data_len_u64 = data_len as u64;
            let data_len_le_bytes = data_len_u64.to_le_bytes();
            let mut byte_idx = 0;
            while byte_idx < __FCE_SDK_FIELD_PREFIX_SIZE {
                manifest[offset + byte_idx] = data_len_le_bytes[byte_idx];
                byte_idx += 1;
            }

            // write data
            let mut byte_idx = 0;
            while byte_idx < data_len {
                manifest[__FCE_SDK_FIELD_PREFIX_SIZE + offset + byte_idx] = data_as_bytes[byte_idx];
                byte_idx += 1;
            }

            (manifest, offset + __FCE_SDK_FIELD_PREFIX_SIZE + data_len)
        }

        const fn generate_manifest() -> [u8; __FCE_MANIFEST_SIZE] {
            let manifest: [u8; __FCE_MANIFEST_SIZE] = [0; __FCE_MANIFEST_SIZE];

            let offset = 0;
            let (manifest, offset) = __fce_sdk_append_data(manifest, $authors, offset);
            let (manifest, offset) = __fce_sdk_append_data(manifest, $version, offset);
            let (manifest, offset) = __fce_sdk_append_data(manifest, $description, offset);
            let (manifest, offset) = __fce_sdk_append_data(manifest, $repository, offset);
            let (manifest, _) = __fce_sdk_append_data(manifest, __FCE_SDK_BUILD_TIME, offset);

            manifest
        }

        #[cfg(target_arch = "wasm32")]
        #[link_section = "__fluence_wasm_module_manifest"]
        #[doc(hidden)]
        pub static __FCE_WASM_MODULE_MANIFEST: [u8; __FCE_MANIFEST_SIZE] = generate_manifest();
    };

    () => {
        module_manifest!(
            env!("CARGO_PKG_AUTHORS"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_REPOSITORY")
        );
    };
}
