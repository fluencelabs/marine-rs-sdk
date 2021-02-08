#![feature(prelude_import)]
//! The main part of Fluence backend SDK. Contains `export_allocator`, `logger` and `result`
//! modules.
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::needless_doctest_main)]
#![doc(html_root_url = "https://docs.rs/fluence-sdk-main/0.2.18")]
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
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
mod call_parameters {
    use fluence_sdk_macro::fce;
    /// Describes an origin that set an argument.
    pub struct SecurityTetraplet {
        pub peer_pk: String,
        pub service_id: String,
        pub function_name: String,
        pub json_path: String,
    }
    #[cfg(target_arch = "wasm32")]
    #[doc(hidden)]
    #[allow(clippy::all)]
    impl SecurityTetraplet {
        pub fn __fce_generated_serialize(self) -> *const u8 {
            let mut raw_record: Vec<u64> = Vec::new();
            raw_record.push(self.peer_pk.as_ptr() as _);
            raw_record.push(self.peer_pk.len() as _);
            std::mem::forget(self.peer_pk);
            raw_record.push(self.service_id.as_ptr() as _);
            raw_record.push(self.service_id.len() as _);
            std::mem::forget(self.service_id);
            raw_record.push(self.function_name.as_ptr() as _);
            raw_record.push(self.function_name.len() as _);
            std::mem::forget(self.function_name);
            raw_record.push(self.json_path.as_ptr() as _);
            raw_record.push(self.json_path.len() as _);
            std::mem::forget(self.json_path);
            let raw_record_ptr = raw_record.as_ptr();
            std::mem::forget(raw_record);
            raw_record_ptr as _
        }
        pub unsafe fn __fce_generated_deserialize(record_ptr: *const u8) -> Self {
            let raw_record: Vec<u64> = Vec::from_raw_parts(record_ptr as _, 64usize, 64usize);
            let field_0 = unsafe {
                String::from_raw_parts(
                    raw_record[0usize] as _,
                    raw_record[1usize] as _,
                    raw_record[1usize] as _,
                )
            };
            let field_1 = unsafe {
                String::from_raw_parts(
                    raw_record[2usize] as _,
                    raw_record[3usize] as _,
                    raw_record[3usize] as _,
                )
            };
            let field_2 = unsafe {
                String::from_raw_parts(
                    raw_record[4usize] as _,
                    raw_record[5usize] as _,
                    raw_record[5usize] as _,
                )
            };
            let field_3 = unsafe {
                String::from_raw_parts(
                    raw_record[6usize] as _,
                    raw_record[7usize] as _,
                    raw_record[7usize] as _,
                )
            };
            Self {
                peer_pk: field_0,
                service_id: field_1,
                function_name: field_2,
                json_path: field_3,
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[doc(hidden)]
    #[allow(clippy::all)]
    #[link_section = "__fce_generated_section__SecurityTetraplet"]
    pub static __fce_generated_static_global_SecurityTetraplet: [u8; 266usize] = {
        * b"{\"ast_type\":\"Record\",\"name\":\"SecurityTetraplet\",\"fields\":[{\"name\":\"peer_pk\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"service_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"function_name\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"json_path\",\"ty\":{\"Utf8String\":\"ByValue\"}}]}"
    };
    /// This struct contains parameters that would be accessible by Wasm modules.
    pub struct CallParameters {
        /// Peer id of the AIR script initiator.
        pub init_peer_id: String,
        /// Id of the current service.
        pub service_id: String,
        /// Id of the service creator.
        pub service_creator_peer_id: String,
        /// Id of the host which run this service.
        pub host_id: String,
        /// Id of the particle which execution resulted a call this service.
        pub particle_id: String,
        /// Security tetraplets which described origin of the arguments.
        pub tetraplets: Vec<Vec<SecurityTetraplet>>,
    }
    #[cfg(target_arch = "wasm32")]
    #[doc(hidden)]
    #[allow(clippy::all)]
    impl CallParameters {
        pub fn __fce_generated_serialize(self) -> *const u8 {
            let mut raw_record: Vec<u64> = Vec::new();
            raw_record.push(self.init_peer_id.as_ptr() as _);
            raw_record.push(self.init_peer_id.len() as _);
            std::mem::forget(self.init_peer_id);
            raw_record.push(self.service_id.as_ptr() as _);
            raw_record.push(self.service_id.len() as _);
            std::mem::forget(self.service_id);
            raw_record.push(self.service_creator_peer_id.as_ptr() as _);
            raw_record.push(self.service_creator_peer_id.len() as _);
            std::mem::forget(self.service_creator_peer_id);
            raw_record.push(self.host_id.as_ptr() as _);
            raw_record.push(self.host_id.len() as _);
            std::mem::forget(self.host_id);
            raw_record.push(self.particle_id.as_ptr() as _);
            raw_record.push(self.particle_id.len() as _);
            std::mem::forget(self.particle_id);
            unsafe fn __fce_generated_vec_serializer_tetraplets_5(
                arg: Vec<Vec<SecurityTetraplet>>,
            ) -> (u32, u32) {
                unsafe fn __fce_generated_vec_serializer_tetraplets_5_SecurityTetraplet(
                    arg: Vec<SecurityTetraplet>,
                ) -> (u32, u32) {
                    let mut result: Vec<u32> = Vec::with_capacity(arg.len());
                    for value in arg {
                        result.push(value.__fce_generated_serialize() as _);
                    }
                    let result = std::mem::ManuallyDrop::new(result);
                    (result.as_ptr() as _, (4 * result.len()) as _)
                }
                let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
                for value in arg {
                    let (ptr, size) =
                        __fce_generated_vec_serializer_tetraplets_5_SecurityTetraplet(value);
                    result.push(ptr as _);
                    result.push(size as _);
                }
                let result = std::mem::ManuallyDrop::new(result);
                (result.as_ptr() as _, (4 * result.len()) as _)
            }
            let serialized_arg_5 =
                unsafe { __fce_generated_vec_serializer_tetraplets_5(self.tetraplets) };
            raw_record.push(serialized_arg_5.0 as _);
            raw_record.push(serialized_arg_5.1 as _);
            let raw_record_ptr = raw_record.as_ptr();
            std::mem::forget(raw_record);
            raw_record_ptr as _
        }
        pub unsafe fn __fce_generated_deserialize(record_ptr: *const u8) -> Self {
            let raw_record: Vec<u64> = Vec::from_raw_parts(record_ptr as _, 96usize, 96usize);
            let field_0 = unsafe {
                String::from_raw_parts(
                    raw_record[0usize] as _,
                    raw_record[1usize] as _,
                    raw_record[1usize] as _,
                )
            };
            let field_1 = unsafe {
                String::from_raw_parts(
                    raw_record[2usize] as _,
                    raw_record[3usize] as _,
                    raw_record[3usize] as _,
                )
            };
            let field_2 = unsafe {
                String::from_raw_parts(
                    raw_record[4usize] as _,
                    raw_record[5usize] as _,
                    raw_record[5usize] as _,
                )
            };
            let field_3 = unsafe {
                String::from_raw_parts(
                    raw_record[6usize] as _,
                    raw_record[7usize] as _,
                    raw_record[7usize] as _,
                )
            };
            let field_4 = unsafe {
                String::from_raw_parts(
                    raw_record[8usize] as _,
                    raw_record[9usize] as _,
                    raw_record[9usize] as _,
                )
            };
            unsafe fn __fce_generated_vec_deserializer_10(
                offset: u32,
                size: u32,
            ) -> Vec<Vec<SecurityTetraplet>> {
                let size = size / 8;
                unsafe fn __fce_generated_vec_deserializer_10_SecurityTetraplet(
                    offset: u32,
                    size: u32,
                ) -> Vec<SecurityTetraplet> {
                    let size = size / 8;
                    let mut arg: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
                    let mut result = Vec::with_capacity(arg.len());
                    for offset in arg {
                        let value = SecurityTetraplet::__fce_generated_deserialize(offset as _);
                        result.push(value);
                    }
                    result
                }
                let mut arg: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
                let mut result = Vec::with_capacity(arg.len());
                let mut arg = arg.into_iter();
                while let Some(offset) = arg.next() {
                    let size = arg.next().unwrap();
                    let value = __fce_generated_vec_deserializer_10_SecurityTetraplet(
                        offset as _,
                        size as _,
                    );
                    result.push(value);
                }
                result
            }
            let field_5 = unsafe {
                __fce_generated_vec_deserializer_10(
                    raw_record[10usize] as _,
                    raw_record[11usize] as _,
                )
            };
            Self {
                init_peer_id: field_0,
                service_id: field_1,
                service_creator_peer_id: field_2,
                host_id: field_3,
                particle_id: field_4,
                tetraplets: field_5,
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[doc(hidden)]
    #[allow(clippy::all)]
    #[link_section = "__fce_generated_section__CallParameters"]
    pub static __fce_generated_static_global_CallParameters: [u8; 445usize] = {
        * b"{\"ast_type\":\"Record\",\"name\":\"CallParameters\",\"fields\":[{\"name\":\"init_peer_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"service_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"service_creator_peer_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"host_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"particle_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"tetraplets\",\"ty\":{\"Vector\":[{\"Vector\":[{\"Record\":[\"SecurityTetraplet\",\"ByValue\"]},\"ByValue\"]},\"ByValue\"]}}]}"
    };
    /// This functions takes from host current call parameters.
    /// Beware that this implies import function call which takes some time.
    #[cfg(target_arch = "wasm32")]
    pub fn get_call_parameters() -> CallParameters {
        unsafe {
            get_call_raw_parameters();
            let raw_call_parameters = crate::result::get_result_ptr();
            CallParameters::__fce_generated_deserialize(raw_call_parameters as _)
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[link(wasm_import_module = "host")]
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "get_call_parameters"]
        fn get_call_raw_parameters();
    }
}
mod export_allocator {
    use super::log;
    use std::alloc::alloc as global_alloc;
    use std::alloc::dealloc as global_dealloc;
    use std::alloc::Layout;
    /// Allocates memory area of specified size and returns its address.
    /// Returns 0 if supplied size is too long.
    #[no_mangle]
    pub unsafe fn allocate(size: usize) -> usize {
        let layout = match Layout::from_size_align(size, std::mem::align_of::<u8>()) {
            Ok(layout) => layout,
            Err(_) => return 0,
        };
        log({
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["sdk.allocate: ", "\n"],
                &match (&size,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                },
            ));
            res
        });
        global_alloc(layout) as _
    }
    /// Deallocates memory area for provided memory pointer and size.
    /// Does nothing if supplied size is too long.
    #[no_mangle]
    pub unsafe fn deallocate(ptr: *mut u8, size: usize) {
        let layout = match Layout::from_size_align(size, std::mem::align_of::<u8>()) {
            Ok(layout) => layout,
            Err(_) => return,
        };
        log({
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["sdk.deallocate: ", " ", "\n"],
                &match (&ptr, &size) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        });
        global_dealloc(ptr, layout);
    }
}
mod result {
    //! Contains ad-hoc implementations of returning complex data types from function calls
    //! by two global variables that contain pointer and size. Will be refactored after multi-value
    //! support in Wasmer.
    use super::log;
    use std::sync::atomic::AtomicUsize;
    static mut RESULT_PTR: AtomicUsize = AtomicUsize::new(0);
    static mut RESULT_SIZE: AtomicUsize = AtomicUsize::new(0);
    #[no_mangle]
    pub unsafe fn get_result_ptr() -> usize {
        log({
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["sdk.get_result_ptr, returns ", "\n"],
                &match (&*RESULT_PTR.get_mut(),) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        });
        *RESULT_PTR.get_mut()
    }
    #[no_mangle]
    pub unsafe fn get_result_size() -> usize {
        log({
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["sdk.get_result_size, returns ", "\n"],
                &match (&*RESULT_SIZE.get_mut(),) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        });
        *RESULT_SIZE.get_mut()
    }
    #[no_mangle]
    pub unsafe fn set_result_ptr(ptr: usize) {
        log({
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["sdk.set_result_ptr: ", "\n"],
                &match (&ptr,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        });
        *RESULT_PTR.get_mut() = ptr;
    }
    #[no_mangle]
    pub unsafe fn set_result_size(size: usize) {
        log({
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["sdk.set_result_size: ", "\n"],
                &match (&size,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        });
        *RESULT_SIZE.get_mut() = size;
    }
}
pub use call_parameters::CallParameters;
pub use call_parameters::SecurityTetraplet;
#[cfg(target_arch = "wasm32")]
pub use call_parameters::get_call_parameters;
pub use export_allocator::allocate;
pub use export_allocator::deallocate;
pub use result::get_result_ptr;
pub use result::get_result_size;
pub use result::set_result_ptr;
pub use result::set_result_size;
#[allow(unused_variables)]
pub(crate) fn log<S: AsRef<str>>(msg: S) {}
