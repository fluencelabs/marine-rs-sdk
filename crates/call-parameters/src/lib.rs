/*
 * Marine Rust SDK
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

#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
use marine_macro::marine;

use serde::Serialize;
use serde::Deserialize;

/// Describes an origin that set corresponding value.
#[cfg_attr(all(target_arch = "wasm32", feature = "marine-abi"), marine)]
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct SecurityTetraplet {
    /// Id of a peer where corresponding value was set.
    pub peer_pk: String,

    /// Id of a service that set corresponding value.
    pub service_id: String,

    /// Name of a function that returned corresponding value.
    pub function_name: String,

    /// Value was produced by applying this `lens` to the output from `call_service`.
    pub lens: String,
}

impl SecurityTetraplet {
    pub fn new(
        peer_pk: impl Into<String>,
        service_id: impl Into<String>,
        function_name: impl Into<String>,
        lens: impl Into<String>,
    ) -> Self {
        Self {
            peer_pk: peer_pk.into(),
            service_id: service_id.into(),
            function_name: function_name.into(),
            lens: lens.into(),
        }
    }

    /// Create a tetraplet for string literals defined in the script
    /// such as variable here `(call ("" "") "" ["variable_1"])`.
    pub fn literal_tetraplet(init_peer_id: impl Into<String>) -> Self {
        Self {
            // these variables represent the initiator peer
            peer_pk: init_peer_id.into(),
            service_id: String::new(),
            function_name: String::new(),
            // lens can't be applied to the string literals
            lens: String::new(),
        }
    }

    pub fn add_lens(&mut self, lens: &str) {
        self.lens.push_str(lens)
    }
}

/// This struct contains parameters that would be accessible by Wasm modules.
#[cfg_attr(all(target_arch = "wasm32", feature = "marine-abi"), marine)]
#[derive(Clone, PartialEq, Default, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct CallParameters {
    /// Parameters of the particle that caused this call.
    pub particle: ParticleParameters,

    /// Id of the current service.
    pub service_id: String,

    /// Id of the service creator.
    pub service_creator_peer_id: String,

    /// PeerId of the peer who hosts worker with this service.
    pub host_id: String,

    /// PeerId of the worker who hosts this service.
    pub worker_id: String,

    /// Security tetraplets which described origin of the arguments.
    pub tetraplets: Vec<Vec<SecurityTetraplet>>,
}

#[cfg_attr(all(target_arch = "wasm32", feature = "marine-abi"), marine)]
#[derive(Clone, PartialEq, Default, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive(check_bytes))]
pub struct ParticleParameters {
    /// Id of the particle which execution resulted a call this service.
    pub id: String,

    /// Peer id of the AIR script initiator.
    pub init_peer_id: String,

    /// Unix timestamp of the particle start time.
    pub timestamp: u64,

    /// Time to live for this particle in milliseconds.
    pub ttl: u32,

    /// AIR script in this particle.
    pub script: String,

    /// Signature made by particle initiator -- init_peer_id.
    pub signature: Vec<u8>,

    /// particle.signature signed by host_id -- used for FS access.
    pub token: String,
}

use std::fmt;
impl fmt::Display for SecurityTetraplet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "peer_pk: {}, service_id: {}, function_name: {}, lens: {}",
            self.peer_pk, self.service_id, self.function_name, self.lens
        )
    }
}

/// This functions takes from host current call parameters.
/// Beware that this implies import function call which takes some time.
#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
pub fn get_call_parameters() -> CallParameters {
    // it's safe until it is executed on standard Fluence node with appropriate import function
    unsafe {
        get_call_raw_parameters();
        let raw_call_parameters = crate::internal::get_result_ptr();
        CallParameters::__m_generated_deserialize(raw_call_parameters as _)
    }
}

#[cfg(not(all(target_arch = "wasm32", feature = "marine-abi")))]
pub fn get_call_parameters() -> CallParameters {
    unimplemented!()
}

#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
#[link(wasm_import_module = "__marine_host_api_v3")]
#[allow(improper_ctypes)]
extern "C" {
    // returns serialized current call parameters
    #[link_name = "get_call_parameters"]
    fn get_call_raw_parameters();
}

#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
#[allow(unused_extern_crates)]
extern crate self as marine_rs_sdk;

#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
#[allow(unused_imports)]
mod internal {
    pub(crate) use marine_rs_sdk_main::add_object_to_release;
    pub(crate) use marine_rs_sdk_main::get_result_ptr;
    pub(crate) use marine_rs_sdk_main::get_result_size;
    pub(crate) use marine_rs_sdk_main::set_result_ptr;
    pub(crate) use marine_rs_sdk_main::set_result_size;
}
