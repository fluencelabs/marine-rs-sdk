/*
 * Copyright 2021 Fluence Labs Limited
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

/// This crate is intended to export the marine macro. Along with exporting it from
/// the "root" marine-rs-sdk crate, it needs to allow using the marine macro on
/// structures that then will be used and exported from the marine-rs-sdk crate.
/// Otherwise, it would require cyclical dependency.

pub use marine_macro::marine;
pub use marine_macro::fce;

extern crate self as marine_rs_sdk;

pub mod internal {
    pub use marine_rs_sdk_main::get_result_ptr;
    pub use marine_rs_sdk_main::get_result_size;
    pub use marine_rs_sdk_main::set_result_ptr;
    pub use marine_rs_sdk_main::set_result_size;
    pub use marine_rs_sdk_main::add_object_to_release;
}
