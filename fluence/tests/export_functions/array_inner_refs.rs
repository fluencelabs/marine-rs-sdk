#![allow(improper_ctypes)]

use fluence::fce;

pub fn main() {}

#[fce]
pub fn byte_type(_arg: Vec<u8>) -> Vec<u8> {
    unimplemented!()
}

#[fce]
pub fn inner_arrays_1(_arg: Vec<&Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}

#[fce]
pub fn inner_arrays_2(_arg: Vec<Vec<&Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}

#[fce]
pub fn inner_arrays_3(_arg: Vec<Vec<Vec<&Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}

#[fce]
pub fn inner_arrays_4(_arg: Vec<Vec<Vec<Vec<&u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    unimplemented!()
}
