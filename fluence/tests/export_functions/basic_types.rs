#![allow(improper_ctypes)]

use fluence::fce;

pub fn main() {}

#[fce]
pub fn all_types(
    _arg_0: i8,
    _arg_1: i16,
    _arg_2: i32,
    _arg_3: i64,
    _arg_4: u8,
    _arg_5: u16,
    _arg_6: u32,
    _arg_7: u64,
    _arg_8: f32,
    _arg_9: f64,
    _arg_10: String,
    _arg_11: Vec<u8>,
) -> Vec<u8> {
    unimplemented!()
}

#[fce]
pub fn string_type(_arg: String) -> String {
    unimplemented!()
}

#[fce]
pub fn bytearray_type(_arg: Vec<u8>) -> Vec<u8> {
    unimplemented!()
}

#[fce]
pub fn bool_type(_arg: bool) -> bool {
    unimplemented!()
}

#[fce]
pub fn f32_type(_arg: f32) -> f32 {
    unimplemented!()
}

#[fce]
pub fn f64_type(_arg: f64) -> f64 {
    unimplemented!()
}

#[fce]
pub fn u32_type(_arg: u32) -> u32 {
    unimplemented!()
}

#[fce]
pub fn u64_type(_arg: u64) -> u64 {
    unimplemented!()
}

#[fce]
pub fn i32_type(_arg: i32) -> i32 {
    unimplemented!()
}

#[fce]
pub fn i64_type(_arg: i64) -> i64 {
    unimplemented!()
}

#[fce]
pub fn empty_type() -> String {
    unimplemented!()
}
