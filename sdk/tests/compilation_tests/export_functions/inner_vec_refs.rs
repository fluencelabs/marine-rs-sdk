#![allow(improper_ctypes)]

use marine_rs_sdk::marine;

pub fn main() {}

#[marine]
pub fn inner_arrays_2(_arg: &Vec<Vec<Vec<Vec<u8>>>>) -> &Vec<Vec<&Vec<Vec<u8>>>> {
    unimplemented!()
}

#[marine]
pub fn inner_arrays_3(_arg: &Vec<Vec<Vec<Vec<u8>>>>) -> &Vec<&Vec<&Vec<&Vec<&u8>>>> {
    unimplemented!()
}
