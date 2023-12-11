#[link(wasm_import_module = "test")]
#[cfg(target_arch = "wasm32")]
extern "C" {
    #[link_name = "all_types"]
    fn __m_generated_wrapper_func__all_types(
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
    );
}
#[cfg(not(target_arch = "wasm32"))]
extern "C" {
    #[link_name = "all_types"]
    fn __m_generated_wrapper_func__all_types(
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
        arg_11: Vec<u8>
    ) -> Vec<u8>;
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
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
    arg_11: Vec<u8>
) -> Vec<u8> {
    unsafe {
        let mut arg_10 = std::mem::ManuallyDrop::new(arg_10);
        unsafe fn __m_generated_vec_serializer_arg_11(arg: &Vec<u8>) -> (u32, u32) {
            (arg.as_ptr() as _, arg.len() as _)
        }
        let arg_11 = __m_generated_vec_serializer_arg_11(&arg_11);
        let result = __m_generated_wrapper_func__all_types(
            arg_0,
            arg_1,
            arg_2,
            arg_3,
            arg_4,
            arg_5,
            arg_6,
            arg_7,
            arg_8,
            arg_9,
            arg_10.as_ptr() as _,
            arg_10.len() as _,
            arg_11.0 as _,
            arg_11.1 as _
        );
        std::mem::ManuallyDrop::drop(&mut arg_10);
        unsafe fn __m_generated_vec_deserializer(offset: u32, size: u32) -> Vec<u8> {
            match size {
                0 => Vec::default(),
                n => Vec::from_raw_parts(offset as _, size as _, size as _)
            }
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
    arg_11: Vec<u8>
) -> Vec<u8> {
    unsafe {
        __m_generated_wrapper_func__all_types(
            arg_0, arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, arg_7, arg_8, arg_9, arg_10, arg_11
        )
    }
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__m_generated_section__test"]
pub static __m_generated_static_global_test: [u8; 687usize] = {
    * b"{\"ast_type\":\"ExternMod\",\"namespace\":\"test\",\"imports\":[{\"link_name\":null,\"signature\":{\"name\":\"all_types\",\"arguments\":[{\"name\":\"arg_0\",\"ty\":{\"I8\":\"ByValue\"}},{\"name\":\"arg_1\",\"ty\":{\"I16\":\"ByValue\"}},{\"name\":\"arg_2\",\"ty\":{\"I32\":\"ByValue\"}},{\"name\":\"arg_3\",\"ty\":{\"I64\":\"ByValue\"}},{\"name\":\"arg_4\",\"ty\":{\"U8\":\"ByValue\"}},{\"name\":\"arg_5\",\"ty\":{\"U16\":\"ByValue\"}},{\"name\":\"arg_6\",\"ty\":{\"U32\":\"ByValue\"}},{\"name\":\"arg_7\",\"ty\":{\"U64\":\"ByValue\"}},{\"name\":\"arg_8\",\"ty\":{\"F32\":\"ByValue\"}},{\"name\":\"arg_9\",\"ty\":{\"F64\":\"ByValue\"}},{\"name\":\"arg_10\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"arg_11\",\"ty\":{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]}}],\"output_types\":[{\"Vector\":[{\"U8\":\"ByValue\"},\"ByValue\"]}]}}]}"
};
