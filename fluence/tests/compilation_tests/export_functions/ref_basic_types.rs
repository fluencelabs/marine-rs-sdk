#![allow(improper_ctypes)]

use fluence::marine;

pub fn main() {}

#[marine]
pub fn all_types<'v>(
    _arg_0: &i8,
    _arg_1: &i16,
    _arg_2: &i32,
    _arg_3: &i64,
    _arg_4: &u8,
    _arg_5: &u16,
    _arg_6: &u32,
    _arg_7: &u64,
    _arg_8: &f32,
    _arg_9: &f64,
    _arg_10: &String,
    _arg_11: &'v Vec<u8>,
) -> &'v Vec<u8> {
    unimplemented!()
}

#[marine]
pub fn string_type(_arg: &String) -> &String {
    unimplemented!()
}

#[marine]
pub fn bytearray_type(_arg: &Vec<u8>) -> &Vec<u8> {
    unimplemented!()
}

#[marine]
pub fn bool_type(_arg: &bool) -> &bool {
    unimplemented!()
}

#[marine]
pub fn f32_type(_arg: &f32) -> &f32 {
    unimplemented!()
}

#[marine]
pub fn f64_type(_arg: &f64) -> &f64 {
    unimplemented!()
}

#[marine]
pub fn u32_type(_arg: &u32) -> &u32 {
    unimplemented!()
}

#[marine]
pub fn u64_type(_arg: &u64) -> &u64 {
    unimplemented!()
}

#[marine]
pub fn i32_type(_arg: &i32) -> &i32 {
    unimplemented!()
}

#[marine]
pub fn i64_type(_arg: &i64) -> &i64 {
    unimplemented!()
}

#[marine]
pub fn empty_type() -> &'static String {
    unimplemented!()
}
