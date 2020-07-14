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
#![doc(html_root_url = "https://docs.rs/fluence-sdk-main/0.2.0")]
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

mod export_allocator;
#[cfg(any(feature = "debug", feature = "logger"))]
mod logger;
mod result;

pub use export_allocator::allocate;
pub use export_allocator::deallocate;
#[cfg(any(feature = "debug", feature = "logger"))]
pub use logger::WasmLogger;
pub use result::get_result_ptr;
pub use result::get_result_size;
pub use result::set_result_ptr;
pub use result::set_result_size;

#[allow(unused_variables)]
pub(crate) fn log<S: AsRef<str>>(msg: S) {
    // logs will be printed only if debug feature is enabled
    #[cfg(all(feature = "debug", feature = "logger"))]
    unsafe {
        let msg = msg.as_ref();
        logger::log_utf8_string(msg.as_ptr() as _, msg.len() as _);
    }
}
