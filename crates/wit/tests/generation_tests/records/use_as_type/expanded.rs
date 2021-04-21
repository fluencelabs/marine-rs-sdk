pub fn inner_arrays_2(arg: Vec<Vec<Vec<Vec<TestRecord>>>>) -> Vec<Vec<Vec<Vec<TestRecord>>>> {
    unimplemented!()
}
#[cfg(target_arch = "wasm32")]
#[export_name = "inner_arrays_2"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __fce_generated_wrapper_func_inner_arrays_2(arg_0: u32, arg_1: u32) {
    unsafe fn __fce_generated_vec_deserializer_0(
        offset: u32,
        size: u32
    ) -> Vec<Vec<Vec<Vec<TestRecord>>>> {
        unsafe fn __fce_generated_vec_deserializer_0_Vec_Vec_TestRecord__(
            offset: u32,
            size: u32
        ) -> Vec<Vec<Vec<TestRecord>>> {
            unsafe fn __fce_generated_vec_deserializer_0_Vec_Vec_TestRecord___Vec_TestRecord_(
                offset: u32,
                size: u32
            ) -> Vec<Vec<TestRecord>> {
                unsafe fn __fce_generated_vec_deserializer_0_Vec_Vec_TestRecord___Vec_TestRecord__TestRecord(
                    offset: u32,
                    size: u32
                ) -> Vec<TestRecord> {
                    let mut arg: Vec<u32> = Vec::from_raw_parts(offset as _, size as _, size as _);
                    let mut result = Vec::with_capacity(arg.len());
                    for offset in arg {
                        let value = TestRecord::__fce_generated_deserialize(offset as _);
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
                    let value = __fce_generated_vec_deserializer_0_Vec_Vec_TestRecord___Vec_TestRecord__TestRecord ( offset as _ , size as _ ) ;
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
                let value = __fce_generated_vec_deserializer_0_Vec_Vec_TestRecord___Vec_TestRecord_(
                    offset as _,
                    size as _
                );
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
            let value =
                __fce_generated_vec_deserializer_0_Vec_Vec_TestRecord__(offset as _, size as _);
            result.push(value);
        }
        result
    }
    let converted_arg_0 = __fce_generated_vec_deserializer_0(arg_0 as _, arg_1 as _);
    let result = inner_arrays_2(converted_arg_0);
    unsafe fn __fce_generated_vec_serializer(arg: &Vec<Vec<Vec<Vec<TestRecord>>>>) -> (u32, u32) {
        unsafe fn __fce_generated_vec_serializer_Vec_Vec_TestRecord__(
            arg: &Vec<Vec<Vec<TestRecord>>>
        ) -> (u32, u32) {
            unsafe fn __fce_generated_vec_serializer_Vec_Vec_TestRecord___Vec_TestRecord_(
                arg: &Vec<Vec<TestRecord>>
            ) -> (u32, u32) {
                unsafe fn __fce_generated_vec_serializer_Vec_Vec_TestRecord___Vec_TestRecord__TestRecord(
                    arg: &Vec<TestRecord>
                ) -> (u32, u32) {
                    let mut result: Vec<u32> = Vec::with_capacity(arg.len());
                    for value in arg {
                        result.push(value.__fce_generated_serialize() as _);
                    }
                    let result_ptr = result.as_ptr();
                    let result_len = result.len();
                    fluence::internal::add_object_to_release(Box::new(result));
                    (result_ptr as _, result_len as _)
                }
                let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
                for value in arg {
                    let ( ptr , size ) = __fce_generated_vec_serializer_Vec_Vec_TestRecord___Vec_TestRecord__TestRecord ( & value ) ;
                    result.push(ptr as _);
                    result.push(size as _);
                }
                let result_ptr = result.as_ptr();
                let result_len = result.len() / 2;
                fluence::internal::add_object_to_release(Box::new(result));
                (result_ptr as _, result_len as _)
            }
            let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
            for value in arg {
                let (ptr, size) =
                    __fce_generated_vec_serializer_Vec_Vec_TestRecord___Vec_TestRecord_(&value);
                result.push(ptr as _);
                result.push(size as _);
            }
            let result_ptr = result.as_ptr();
            let result_len = result.len() / 2;
            fluence::internal::add_object_to_release(Box::new(result));
            (result_ptr as _, result_len as _)
        }
        let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
        for value in arg {
            let (ptr, size) = __fce_generated_vec_serializer_Vec_Vec_TestRecord__(&value);
            result.push(ptr as _);
            result.push(size as _);
        }
        let result_ptr = result.as_ptr();
        let result_len = result.len() / 2;
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
#[link_section = "__fce_generated_section__inner_arrays_2"]
pub static __fce_generated_static_global_inner_arrays_2: [u8; 365usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"inner_arrays_2\",\"arguments\":[{\"name\":\"arg\",\"ty\":{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"Record\":[\"TestRecord\",\"ByValue\"]},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]}}],\"output_type\":{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"Vector\":[{\"Record\":[\"TestRecord\",\"ByValue\"]},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]},\"ByValue\"]}}}"
};