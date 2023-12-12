pub fn all_types(
    arg_0: i8,
    arg_1: i16,
    arg_2: i32,
    arg_3: i64,
    arg_4: u8,
    arg_5: u16,
    arg_6: u32,
    arg_7: u64,
    arg_8: f32,
    arg_9: f64,
    arg_10: String,
    arg_11: Vec<u8>,
) -> Vec<u8> {
    unimplemented!()
}
#[cfg(target_arch = "wasm32")]
#[export_name = "all_types"]
#[no_mangle]
#[doc(hidden)]
#[allow(clippy::all)]
pub unsafe fn __m_generated_wrapper_func_all_types(
    arg_0: i8,
    arg_1: i16,
    arg_2: i32,
    arg_3: i64,
    arg_4: u8,
    arg_5: u16,
    arg_6: u32,
    arg_7: u64,
    arg_8: f32,
    arg_9: f64,
    arg_10: u32,
    arg_11: u32,
    arg_12: u32,
    arg_13: u32
) {
    let converted_arg_0 = arg_0 as _;
    let converted_arg_1 = arg_1 as _;
    let converted_arg_2 = arg_2 as _;
    let converted_arg_3 = arg_3 as _;
    let converted_arg_4 = arg_4 as _;
    let converted_arg_5 = arg_5 as _;
    let converted_arg_6 = arg_6 as _;
    let converted_arg_7 = arg_7 as _;
    let converted_arg_8 = arg_8 as _;
    let converted_arg_9 = arg_9 as _;
    let converted_arg_10 = match arg_11 {
        0 => String::default(),
        _ => String::from_raw_parts(arg_10 as _, arg_11 as _, arg_11 as _)
    };
    unsafe fn __m_generated_vec_deserializer_12(offset: u32, size: u32) -> Vec<u8> {
        match size {
            0 => Vec::default(),
            _ => Vec::from_raw_parts(offset as _, size as _, size as _)
        }
    }
    let converted_arg_12 = __m_generated_vec_deserializer_12(arg_12 as _, arg_13 as _);
    let result = all_types(
        converted_arg_0,
        converted_arg_1,
        converted_arg_2,
        converted_arg_3,
        converted_arg_4,
        converted_arg_5,
        converted_arg_6,
        converted_arg_7,
        converted_arg_8,
        converted_arg_9,
        converted_arg_10,
        converted_arg_12
    );
    unsafe fn __m_generated_vec_serializer(arg: &Vec<u8>) -> (u32, u32) {
        (arg.as_ptr() as _, arg.len() as _)
    }
    {
        let (serialized_vec_ptr, serialized_vec_size) = __m_generated_vec_serializer(&result);
        marine_rs_sdk::internal::set_result_ptr(serialized_vec_ptr as _);
        marine_rs_sdk::internal::set_result_size(serialized_vec_size as _);
    }
    marine_rs_sdk::internal::add_object_to_release(Box::new(result));
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__m_generated_section__all_types"]
pub static __m_generated_static_global_all_types: [u8; 636usize] = {
    * b"{\"ast_type\":\"Function\",\"signature\":{\"name\":\"all_types\",\"arguments\":[{\"name\":\"arg_0\",\"ty\":{\"I8\":\"ByValue\"}},{\"name\":\"arg_1\",\"ty\":{\"I16\":\"ByValue\"}},{\"name\":\"arg_2\",\"ty\":{\"I32\":\"ByValue\"}},{\"name\":\"arg_3\",\"ty\":{\"I64\":\"ByValue\"}},{\"name\":\"arg_4\",\"ty\":{\"U8\":\"ByValue\"}},{\"name\":\"arg_5\",\"ty\":{\"U16\":\"ByValue\"}},{\"name\":\"arg_6\",\"ty\":{\"U32\":\"ByValue\"}},{\"name\":\"arg_7\",\"ty\":{\"U64\":\"ByValue\"}},{\"name\":\"arg_8\",\"ty\":{\"F32\":\"ByValue\"}},{\"name\":\"arg_9\",\"ty\":{\"F64\":\"ByValue\"}},{\"name\":\"arg_10\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"arg_11\",\"ty\":{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]}}],\"output_types\":[{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]}]}}"
};
