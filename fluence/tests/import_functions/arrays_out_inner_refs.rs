#![allow(improper_ctypes)]

use fluence::fce;

pub fn main() {}

#[fce]
#[link(wasm_import_module = "arrays_passing_effector")]
extern "C" {
    #[fce]
    pub fn func_1() -> &String;

    #[fce]
    pub fn func_2() -> &Vec<Vec<Vec<Vec<u8>>>>;

    #[fce]
    pub fn func_3() -> Vec<&Vec<Vec<Vec<u8>>>>;

    #[fce]
    pub fn func_4() -> Vec<Vec<&Vec<Vec<u8>>>>;

    #[fce]
    pub fn func_5() -> Vec<Vec<Vec<&Vec<u8>>>>;

    #[fce]
    pub fn func_6() -> Vec<Vec<Vec<Vec<&u8>>>>;
}
