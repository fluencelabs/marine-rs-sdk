#![allow(improper_ctypes)]

use fluence::marine;

pub fn main() {}

#[marine]
pub fn byte_type(_arg: &Vec<u8>) -> &Vec<u8> {
    unimplemented!()
}

#[marine]
pub fn inner_arrays_1(_arg: &Vec<Vec<Vec<Vec<u8>>>>) -> &Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}

#[marine]
#[derive(Default)]
pub struct TestRecord {
    pub field_0: i32,
    pub field_1: Vec<Vec<u8>>,
}

#[marine]
pub fn inner_arrays_4(_arg: &Vec<Vec<Vec<Vec<TestRecord>>>>) -> &Vec<Vec<Vec<Vec<TestRecord>>>> {
    unimplemented!()
}

#[marine]
pub fn string_type(_arg: &Vec<String>) -> &Vec<String> {
    unimplemented!()
}

#[marine]
pub fn bool_type(_arg: &Vec<bool>) -> &Vec<bool> {
    unimplemented!()
}

#[marine]
pub fn f32_type(_arg: &Vec<f32>) -> &Vec<f32> {
    unimplemented!()
}

#[marine]
pub fn f64_type(_arg: &Vec<f64>) -> &Vec<f64> {
    unimplemented!()
}

#[marine]
pub fn u32_type(_arg: &Vec<u32>) -> &Vec<u32> {
    unimplemented!()
}

#[marine]
pub fn u64_type(_arg: &Vec<u64>) -> &Vec<u64> {
    unimplemented!()
}

#[marine]
pub fn i32_type(_arg: &Vec<i32>) -> &Vec<i32> {
    unimplemented!()
}

#[marine]
pub fn i64_type(_arg: &Vec<i64>) -> &Vec<i64> {
    unimplemented!()
}

#[marine]
pub fn empty_type() -> &'static Vec<String> {
    unimplemented!()
}
