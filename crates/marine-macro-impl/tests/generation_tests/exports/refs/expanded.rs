pub fn test_array_refs(arg: &Vec<Vec<String>>) -> &Vec<Vec<Vec<Vec<String>>>> {
    unimplemented!()
}
#[cfg(target_arch = "wasm32")]
#[export_name = "test_array_refs"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __m_generated_wrapper_func_test_array_refs(arg_0: u32, arg_1: u32) {
    unsafe fn __m_generated_vec_deserializer_0(offset: u32, size: u32) -> Vec<Vec<String>> {
        unsafe fn __m_generated_vec_deserializer_0_String(offset: u32, size: u32) -> Vec<String> {
            let vec_passing_size = 2;
            let mut arg: Vec<u32> =
                Vec::from_raw_parts(offset as _, (vec_passing_size * size) as _, (vec_passing_size * size) as _);
            let mut arg = arg.into_iter();
            let mut result = Vec::with_capacity(arg.len() / 2);
            while let Some(offset) = arg.next() {
                let size = arg.next().unwrap();
                let value = match size {
                    0 => String::default(),
                    n => String::from_raw_parts(offset as _, size as _, size as _)
                };
                result.push(value);
            }
            result
        }
        let vec_passing_size = 2;
        let mut arg: Vec<u32> = Vec::from_raw_parts(offset as _, (vec_passing_size * size) as _, (vec_passing_size * size) as _);
        let mut result = Vec::with_capacity(arg.len());
        let mut arg = arg.into_iter();
        while let Some(offset) = arg.next() {
            let size = arg.next().unwrap();
            let value = __m_generated_vec_deserializer_0_String(offset as _, size as _);
            result.push(value);
        }
        result
    }
    let converted_arg_0 = __m_generated_vec_deserializer_0(arg_0 as _, arg_1 as _);
    let result = test_array_refs(&converted_arg_0);
    unsafe fn __m_generated_vec_serializer(arg: &Vec<Vec<Vec<Vec<String>>>>) -> (u32, u32) {
        unsafe fn __m_generated_vec_serializer_Vec_Vec_String__(
            arg: &Vec<Vec<Vec<String>>>
        ) -> (u32, u32) {
            unsafe fn __m_generated_vec_serializer_Vec_Vec_String___Vec_String_(
                arg: &Vec<Vec<String>>
            ) -> (u32, u32) {
                unsafe fn __m_generated_vec_serializer_Vec_Vec_String___Vec_String__String(
                    arg: &Vec<String>
                ) -> (u32, u32) {
                    let mut result: Vec<u32> = Vec::with_capacity(arg.len());
                    for value in arg {
                        result.push(value.as_ptr() as _);
                        result.push(value.len() as _);
                    }
                    let result_ptr = result.as_ptr();
                    let result_len = result.len() / 2;
                    marine_rs_sdk::internal::add_object_to_release(Box::new(result));
                    (result_ptr as _, result_len as _)
                }
                let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
                for value in arg {
                    let (ptr, size) =
                        __m_generated_vec_serializer_Vec_Vec_String___Vec_String__String(&value);
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
                    __m_generated_vec_serializer_Vec_Vec_String___Vec_String_(&value);
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
            let (ptr, size) = __m_generated_vec_serializer_Vec_Vec_String__(&value);
            result.push(ptr as _);
            result.push(size as _);
        }
        let result_ptr = result.as_ptr();
        let result_len = result.len() / 2;
        marine_rs_sdk::internal::add_object_to_release(Box::new(result));
        (result_ptr as _, result_len as _)
    }
    {
        let (serialized_vec_ptr, serialized_vec_size) = __m_generated_vec_serializer(&result);
        marine_rs_sdk::internal::set_result_ptr(serialized_vec_ptr as _);
        marine_rs_sdk::internal::set_result_size(serialized_vec_size as _);
    }
    marine_rs_sdk::internal::add_object_to_release(Box::new(converted_arg_0));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__m_generated_section__test_array_refs"]
pub static __m_generated_static_global_test_array_refs: [u8; 297usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"test_array_refs\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"Vector\":[{\"Vector\":[{\"Utf8String\":\"ByValue\"},\"ByValue\"]},\"ByRef\"]}}],\"output_types\":[{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"Utf8String\":\"ByValue\"},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]},\"ByRef\"]}]}}"
};
