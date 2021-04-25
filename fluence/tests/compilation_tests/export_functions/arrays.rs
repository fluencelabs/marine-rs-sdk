#![allow(improper_ctypes)]

use fluence::fce;

pub fn main() {}

#[fce]
pub fn byte_type(_arg: Vec<u8>) -> Vec<u8> {
    unimplemented!()
}

#[fce]
pub fn inner_arrays_1(_arg: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}

#[fce]
#[derive(Default)]
pub struct TestRecord {
    pub field_0: i32,
    pub field_1: Vec<Vec<u8>>,
}

#[fce]
pub fn inner_arrays_2(_arg: Vec<Vec<Vec<Vec<TestRecord>>>>) -> Vec<Vec<Vec<Vec<TestRecord>>>> {
    unimplemented!()
}

#[fce]
pub fn string_type(_arg: Vec<String>) -> Vec<String> {
    unimplemented!()
}

#[fce]
pub fn bool_type(_arg: Vec<bool>) -> Vec<bool> {
    unimplemented!()
}

#[fce]
pub fn f32_type(_arg: Vec<f32>) -> Vec<f32> {
    unimplemented!()
}

#[fce]
pub fn f64_type(_arg: Vec<f64>) -> Vec<f64> {
    unimplemented!()
}

#[fce]
pub fn u32_type(_arg: Vec<u32>) -> Vec<u32> {
    unimplemented!()
}

#[fce]
pub fn u64_type(_arg: Vec<u64>) -> Vec<u64> {
    unimplemented!()
}

#[fce]
pub fn i32_type(_arg: Vec<i32>) -> Vec<i32> {
    unimplemented!()
}

#[fce]
pub fn i64_type(_arg: Vec<i64>) -> Vec<i64> {
    unimplemented!()
}

#[fce]
pub fn empty_type() -> Vec<String> {
    unimplemented!()
}
