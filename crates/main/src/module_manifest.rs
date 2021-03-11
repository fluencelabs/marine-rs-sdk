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

#[macro_export]
macro_rules! module_manifest {
    ($authors:expr, $version:expr, $description:expr, $repository:expr) => {
        const AUTHORS_SIZE: usize = $authors.as_bytes().len();
        const VERSION_SIZE: usize = $version.as_bytes().len();
        const DESCRIPTION_SIZE: usize = $description.as_bytes().len();
        const REPOSITORY_SIZE: usize = $repository.as_bytes().len();
        const FEILD_PREFIX_SIZE: usize = std::mem::size_of::<u64>();

        const MANIFEST_SIZE: usize = AUTHORS_SIZE
            + VERSION_SIZE
            + DESCRIPTION_SIZE
            + REPOSITORY_SIZE
            + FEILD_PREFIX_SIZE * 4;

        const fn append_data(
            mut manifest: [u8; MANIFEST_SIZE],
            data: &'static str,
            offset: usize,
        ) -> ([u8; MANIFEST_SIZE], usize) {
            let data_as_bytes = data.as_bytes();
            let data_len = data_as_bytes.len();

            // write data prefix with data size in LE
            let data_len_u64 = data_len as u64;
            let data_len_le_bytes = data_len_u64.to_le_bytes();
            let mut byte_id = 0;
            while byte_id < FEILD_PREFIX_SIZE {
                manifest[offset + byte_id] = data_len_le_bytes[byte_id];
                byte_id += 1;
            }

            // write data
            let mut byte_id = 0;
            while byte_id < data_len {
                manifest[FEILD_PREFIX_SIZE + offset + byte_id] = data_as_bytes[byte_id];
                byte_id += 1;
            }

            (manifest, offset + FEILD_PREFIX_SIZE + data_len)
        }

        const fn generate_manifest() -> [u8; MANIFEST_SIZE] {
            let authors = $authors;
            let version = $version;
            let description = $description;
            let repository = $repository;

            let manifest: [u8; MANIFEST_SIZE] = [0; MANIFEST_SIZE];

            let offset = 0;
            let (manifest, offset) = append_data(manifest, authors, offset);
            let (manifest, offset) = append_data(manifest, version, offset);
            let (manifest, offset) = append_data(manifest, description, offset);
            let (manifest, _) = append_data(manifest, repository, offset);

            manifest
        }

        #[cfg(target_arch = "wasm32")]
        #[link_section = "__fluence_wasm_module_manifest"]
        #[doc(hidden)]
        pub static WASM_MODULE_MANIFEST: [u8; MANIFEST_SIZE] = generate_manifest();
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
