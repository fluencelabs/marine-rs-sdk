/*
 * Copyright 2018 Fluence Labs Limited
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

//! The main part of Fluence backend SDK. Contains `export_allocator`, `logger` and `result`
//! modules.

#![allow(clippy::missing_safety_doc)]
#![allow(clippy::needless_doctest_main)]
#![doc(html_root_url = "https://docs.rs/marine-rs-sdk-main/0.10.2")] // x-release-please-version
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]
#![warn(rust_2018_idioms)]

#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
mod export_allocator;
#[cfg(any(feature = "debug", feature = "logger"))]
mod logger;
mod module_manifest;
#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
mod result;
mod sdk_version_embedder;

#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
pub use export_allocator::allocate;

#[cfg(feature = "logger")]
pub use logger::{WasmLoggerBuilder, TargetMap, WASM_LOG_ENV_NAME};

#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
pub use result::{
    get_result_ptr, get_result_size, set_result_ptr, set_result_size, release_objects,
    add_object_to_release,
};

pub use module_manifest::MANIFEST_SECTION_NAME;
pub use sdk_version_embedder::VERSION_SECTION_NAME;

// these logs will be printed only if debug feature is enabled
#[macro_export]
macro_rules! debug_log {
    ($msg_generator:expr) => {
        #[cfg(feature = "debug")]
        {
            let level = log::Level::Info as i32;
            let target = 0i32;
            let msg = $msg_generator;
            $crate::logger::log_utf8_string(level, target, msg.as_ptr() as i32, msg.len() as i32);
        }
    };
}
