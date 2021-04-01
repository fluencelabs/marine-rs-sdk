#![allow(improper_ctypes)]

use fluence::fce;

pub fn main() {}

#[fce]
#[derive(Default)]
pub struct TestRecord {
    pub field_0: i32,
    pub field_1: Vec<Vec<u8>>,
}

#[fce]
#[link(wasm_import_module = "arrays_passing_effector")]
extern "C" {
    pub fn inner_arrays_1(arg: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>>;

    pub fn inner_arrays_2(
        arg: Vec<Vec<Vec<Vec<TestRecord>>>>,
    ) -> Vec<Vec<Vec<Vec<TestRecord>>>>;

    pub fn string_type(arg: Vec<String>) -> Vec<String>;

    pub fn byte_type(arg: Vec<u8>) -> Vec<u8>;

    pub fn f32_type(arg: Vec<f32>) -> Vec<f32>;

    pub fn f64_type(arg: Vec<f64>) -> Vec<f64>;

    pub fn u32_type(arg: Vec<u32>) -> Vec<u32>;

    pub fn u64_type(arg: Vec<u64>) -> Vec<u64>;

    pub fn i32_type(arg: Vec<i32>) -> Vec<i32>;

    pub fn i64_type(arg: Vec<i64>) -> Vec<i64>;

    pub fn empty_type() -> Vec<String>;
}
