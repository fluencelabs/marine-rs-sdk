pub fn inner_arrays_1(arg: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}
#[cfg(target_arch = "wasm32")]
#[export_name = "inner_arrays_1"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_inner_arrays_1(arg_0: u32, arg_1: u32) {
    unsafe fn __fce_generated_vec_deserializer_0(offset: u32, size: u32) -> Vec<Vec<Vec<Vec<u8>>>> {
        unsafe fn __fce_generated_vec_deserializer_0_Vec_Vec_u8__(
            offset: u32,
            size: u32
        ) -> Vec<Vec<Vec<u8>>> {
            unsafe fn __fce_generated_vec_deserializer_0_Vec_Vec_u8___Vec_u8_(
                offset: u32,
                size: u32
            ) -> Vec<Vec<u8>> {
                unsafe fn __fce_generated_vec_deserializer_0_Vec_Vec_u8___Vec_u8__u8(
                    offset: u32,
                    size: u32
                ) -> Vec<u8> {
                    Vec::from_raw_parts(offset as _, size as _, size as _)
                }
                let vec_passing_size = 2;
                let mut arg: Vec<u32> =
                    Vec::from_raw_parts(offset as _, (vec_passing_size * size) as _, (vec_passing_size * size) as _);
                let mut result = Vec::with_capacity(arg.len());
                let mut arg = arg.into_iter();
                while let Some(offset) = arg.next() {
                    let size = arg.next().unwrap();
                    let value = __fce_generated_vec_deserializer_0_Vec_Vec_u8___Vec_u8__u8(
                        offset as _,
                        size as _
                    );
                    result.push(value);
                }
                result
            }
            let vec_passing_size = 2;
            let mut arg: Vec<u32> =
                Vec::from_raw_parts(offset as _, (vec_passing_size * size) as _, (vec_passing_size * size) as _);
            let mut result = Vec::with_capacity(arg.len());
            let mut arg = arg.into_iter();
            while let Some(offset) = arg.next() {
                let size = arg.next().unwrap();
                let value =
                    __fce_generated_vec_deserializer_0_Vec_Vec_u8___Vec_u8_(offset as _, size as _);
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
            let value = __fce_generated_vec_deserializer_0_Vec_Vec_u8__(offset as _, size as _);
            result.push(value);
        }
        result
    }
    let converted_arg_0 = __fce_generated_vec_deserializer_0(arg_0 as _, arg_1 as _);
    let result = inner_arrays_1(converted_arg_0);
    unsafe fn __fce_generated_vec_serializer(arg: &Vec<Vec<Vec<Vec<u8>>>>) -> (u32, u32) {
        unsafe fn __fce_generated_vec_serializer_Vec_Vec_u8__(
            arg: &Vec<Vec<Vec<u8>>>
        ) -> (u32, u32) {
            unsafe fn __fce_generated_vec_serializer_Vec_Vec_u8___Vec_u8_(
                arg: &Vec<Vec<u8>>
            ) -> (u32, u32) {
                unsafe fn __fce_generated_vec_serializer_Vec_Vec_u8___Vec_u8__u8(
                    arg: &Vec<u8>
                ) -> (u32, u32) {
                    (arg.as_ptr() as _, arg.len() as _)
                }
                let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
                for value in arg {
                    let (ptr, size) =
                        __fce_generated_vec_serializer_Vec_Vec_u8___Vec_u8__u8(&value);
                    result.push(ptr as _);
                    result.push(size as _);
                }
                let result_ptr = result.as_ptr();
                let result_len = result.len();
                fluence::internal::add_object_to_release(Box::new(result));
                (result_ptr as _, result_len as _)
            }
            let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
            for value in arg {
                let (ptr, size) = __fce_generated_vec_serializer_Vec_Vec_u8___Vec_u8_(&value);
                result.push(ptr as _);
                result.push(size as _);
            }
            let result_ptr = result.as_ptr();
            let result_len = result.len();
            fluence::internal::add_object_to_release(Box::new(result));
            (result_ptr as _, result_len as _)
        }
        let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
        for value in arg {
            let (ptr, size) = __fce_generated_vec_serializer_Vec_Vec_u8__(&value);
            result.push(ptr as _);
            result.push(size as _);
        }
        let result_ptr = result.as_ptr();
        let result_len = result.len();
        fluence::internal::add_object_to_release(Box::new(result));
        (result_ptr as _, result_len as _)
    }
    {
        let (serialized_vec_ptr, serialized_vec_size) = __fce_generated_vec_serializer(&result);
        fluence::internal::set_result_ptr(serialized_vec_ptr as _);
        fluence::internal::set_result_size(serialized_vec_size as _);
    }
    fluence::internal::add_object_to_release(Box::new(result));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__inner_arrays_1"]
pub static __fce_generated_static_global_inner_arrays_1: [u8; 327usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"inner_arrays_1\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]}}],\"output_type\":{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]}}}"
};
