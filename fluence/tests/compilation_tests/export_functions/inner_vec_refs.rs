#![allow(improper_ctypes)]

#[fce]
pub fn inner_arrays_2(_arg: &Vec<Vec<Vec<Vec<u8>>>>) -> &Vec<Vec<&Vec<Vec<u8>>>> {
    unimplemented!()
}

#[fce]
pub fn inner_arrays_3(_arg: &Vec<Vec<Vec<Vec<u8>>>>) -> &Vec<&Vec<&Vec<&Vec<&u8>>>> {
    unimplemented!()
}
