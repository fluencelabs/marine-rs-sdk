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

use marine_macro_export::marine;
pub use polyplets::SecurityTetraplet;

use serde::Serialize;
use serde::Deserialize;

/// This struct contains parameters that would be accessible by Wasm modules.
#[marine]
#[derive(Clone, PartialEq, Default, Eq, Debug, Serialize, Deserialize)]
pub struct CallParameters {
    /// Peer id of the AIR script initiator.
    pub init_peer_id: String,

    /// Id of the current service.
    pub service_id: String,

    /// Id of the service creator.
    pub service_creator_peer_id: String,

    /// PeerId of the peer who hosts this service.
    pub host_id: String,

    /// Id of the particle which execution resulted a call this service.
    pub particle_id: String,

    /// Security tetraplets which described origin of the arguments.
    pub tetraplets: Vec<Vec<SecurityTetraplet>>,
}

/// This functions takes from host current call parameters.
/// Beware that this implies import function call which takes some time.
#[cfg(target_arch = "wasm32")]
pub fn get_call_parameters() -> CallParameters {
    // it's safe until it is executed on standard Fluence node with appropriate import function
    unsafe {
        get_call_raw_parameters();
        let raw_call_parameters = crate::internal::get_result_ptr();
        CallParameters::__m_generated_deserialize(raw_call_parameters as _)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_call_parameters() -> CallParameters {
    unimplemented!()
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "host")]
#[allow(improper_ctypes)]
extern "C" {
    // returns serialized current call parameters
    #[link_name = "get_call_parameters"]
    fn get_call_raw_parameters();
}
