#![allow(improper_ctypes)]

use marine_rs_sdk::marine;

fn main() {}

#[marine]
#[link(wasm_import_module = "arguments_passing_effector")]
extern "C" {
    pub fn all_ref_types(
        arg_0: &i8,
        arg_1: &i16,
        arg_2: &i32,
        arg_3: &i64,
        arg_4: &u8,
        arg_5: &u16,
        arg_6: &u32,
        arg_7: &u64,
        arg_8: &f32,
        arg_9: &f64,
        arg_10: &String,
        arg_11: &Vec<u8>,
    ) -> Vec<u8>;

    pub fn string_ref_type(arg: &String) -> String;

    pub fn str_type(arg: &str) -> String;

    pub fn bytearray_ref_type(arg: &Vec<u8>) -> Vec<u8>;

    pub fn bool_ref_type(arg: &bool) -> bool;

    pub fn f32_ref_type(arg: &f32) -> f32;

    pub fn f64_ref_type(arg: &f64) -> f64;

    pub fn u32_ref_type(arg: &u32) -> u32;

    pub fn u64_ref_type(arg: &u64) -> u64;

    pub fn i32_ref_type(arg: &i32) -> i32;

    pub fn i64_ref_type(arg: &i64) -> i64;
}
