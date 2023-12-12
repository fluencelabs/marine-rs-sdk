#[link(wasm_import_module = "test")]
#[cfg(target_arch = "wasm32")]
extern "C" {
    #[link_name = "inner_arrays_1"]
    fn __m_generated_wrapper_func__inner_arrays_1(arg_0: u32, arg_1: u32);
}
#[cfg(not(target_arch = "wasm32"))]
extern "C" {
    #[link_name = "inner_arrays_1"]
    fn __m_generated_wrapper_func__inner_arrays_1(
        arg: Vec<Vec<Vec<Vec<u8>>>>
    ) -> Vec<Vec<Vec<Vec<u8>>>>;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
pub fn inner_arrays_1(arg_0: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unsafe {
        unsafe fn __m_generated_vec_serializer_arg_0(arg: &Vec<Vec<Vec<Vec<u8>>>>) -> (u32, u32) {
            unsafe fn __m_generated_vec_serializer_arg_0_Vec_Vec_u8__(
                arg: &Vec<Vec<Vec<u8>>>
            ) -> (u32, u32) {
                unsafe fn __m_generated_vec_serializer_arg_0_Vec_Vec_u8___Vec_u8_(
                    arg: &Vec<Vec<u8>>
                ) -> (u32, u32) {
                    unsafe fn __m_generated_vec_serializer_arg_0_Vec_Vec_u8___Vec_u8__u8(
                        arg: &Vec<u8>
                    ) -> (u32, u32) {
                        (arg.as_ptr() as _, arg.len() as _)
                    }
                    let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
                    for value in arg {
                        let (ptr, size) =
                            __m_generated_vec_serializer_arg_0_Vec_Vec_u8___Vec_u8__u8(&value);
                        result.push(ptr as _);
                        result.push(size as _);
                    }
                    let result_ptr = result.as_ptr();
                    let result_len = result.len() / 2;
                    marine_rs_sdk::internal::add_object_to_release(Box::new(result));
                    (result_ptr as _, result_len as _)
                }
                let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
                for value in arg {
                    let (ptr, size) =
                        __m_generated_vec_serializer_arg_0_Vec_Vec_u8___Vec_u8_(&value);
                    result.push(ptr as _);
                    result.push(size as _);
                }
                let result_ptr = result.as_ptr();
                let result_len = result.len() / 2;
                marine_rs_sdk::internal::add_object_to_release(Box::new(result));
                (result_ptr as _, result_len as _)
            }
            let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
            for value in arg {
                let (ptr, size) = __m_generated_vec_serializer_arg_0_Vec_Vec_u8__(&value);
                result.push(ptr as _);
                result.push(size as _);
            }
            let result_ptr = result.as_ptr();
            let result_len = result.len() / 2;
            marine_rs_sdk::internal::add_object_to_release(Box::new(result));
            (result_ptr as _, result_len as _)
        }
        let arg_0 = __m_generated_vec_serializer_arg_0(&arg_0);
        let result = __m_generated_wrapper_func__inner_arrays_1(arg_0.0 as _, arg_0.1 as _);
        unsafe fn __m_generated_vec_deserializer(offset: u32, size: u32) -> Vec<Vec<Vec<Vec<u8>>>> {
            unsafe fn __m_generated_vec_deserializer_Vec_Vec_u8__(
                offset: u32,
                size: u32
            ) -> Vec<Vec<Vec<u8>>> {
                unsafe fn __m_generated_vec_deserializer_Vec_Vec_u8___Vec_u8_(
                    offset: u32,
                    size: u32
                ) -> Vec<Vec<u8>> {
                    unsafe fn __m_generated_vec_deserializer_Vec_Vec_u8___Vec_u8__u8(
                        offset: u32,
                        size: u32
                    ) -> Vec<u8> {
                        match size {
                            0 => Vec::default(),
                            _ => Vec::from_raw_parts(offset as _, size as _, size as _)
                        }
                    }
                    let vec_passing_size = 2;
                    let mut arg: Vec<u32> = Vec::from_raw_parts(
                        offset as _,
                        (vec_passing_size * size) as _,
                        (vec_passing_size * size) as _
                    );
                    let mut result = Vec::with_capacity(arg.len());
                    let mut arg = arg.into_iter();
                    while let Some(offset) = arg.next() {
                        let size = arg.next().unwrap();
                        let value = __m_generated_vec_deserializer_Vec_Vec_u8___Vec_u8__u8(
                            offset as _,
                            size as _
                        );
                        result.push(value);
                    }
                    result
                }
                let vec_passing_size = 2;
                let mut arg: Vec<u32> = Vec::from_raw_parts(
                    offset as _,
                    (vec_passing_size * size) as _,
                    (vec_passing_size * size) as _
                );
                let mut result = Vec::with_capacity(arg.len());
                let mut arg = arg.into_iter();
                while let Some(offset) = arg.next() {
                    let size = arg.next().unwrap();
                    let value =
                        __m_generated_vec_deserializer_Vec_Vec_u8___Vec_u8_(offset as _, size as _);
                    result.push(value);
                }
                result
            }
            let vec_passing_size = 2;
            let mut arg: Vec<u32> = Vec::from_raw_parts(
                offset as _,
                (vec_passing_size * size) as _,
                (vec_passing_size * size) as _
            );
            let mut result = Vec::with_capacity(arg.len());
            let mut arg = arg.into_iter();
            while let Some(offset) = arg.next() {
                let size = arg.next().unwrap();
                let value = __m_generated_vec_deserializer_Vec_Vec_u8__(offset as _, size as _);
                result.push(value);
            }
            result
        }
        __m_generated_vec_deserializer(
            marine_rs_sdk::internal::get_result_ptr() as _,
            marine_rs_sdk::internal::get_result_size() as _,
        )
    }
}
#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
#[allow(clippy::all)]
pub fn inner_arrays_1(arg_0: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unsafe { __m_generated_wrapper_func__inner_arrays_1(arg_0) }
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__m_generated_section__test"]
pub static __m_generated_static_global_test: [u8; 381usize] = {
    * b"{\"ast_type\":\"ExternMod\",\"namespace\":\"test\",\"imports\":[{\"link_name\":null,\"signature\":{\"name\":\"inner_arrays_1\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]}}],\"output_types\":[{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]}]}}]}"
};