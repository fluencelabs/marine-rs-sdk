#![allow(improper_ctypes)]

use fluence::marine;

pub fn main() {}

#[marine]
pub fn byte_type(_arg: Vec<u8>) -> Vec<u8> {
    unimplemented!()
}

#[marine]
pub fn inner_arrays_1(_arg: Vec<&Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}

#[marine]
pub fn inner_arrays_2(_arg: Vec<Vec<&Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}

#[marine]
pub fn inner_arrays_3(_arg: Vec<Vec<Vec<&Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}

#[marine]
pub fn inner_arrays_4(_arg: Vec<Vec<Vec<Vec<&u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}

#[marine]
pub fn inner_arrays_5(_arg1: i32, _arg2: Vec<Vec<Vec<&Vec<u8>>>>, _arg3: i32) -> Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}
