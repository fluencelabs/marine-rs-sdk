#![allow(improper_ctypes)]

use fluence::marine;

pub fn main() {}

#[marine]
#[link(wasm_import_module = "arrays_passing_effector")]
extern "C" {
    #[marine]
    pub fn func_1() -> &String;

    #[marine]
    pub fn func_2() -> &Vec<Vec<Vec<Vec<u8>>>>;

    #[marine]
    pub fn func_3() -> Vec<&Vec<Vec<Vec<u8>>>>;

    #[marine]
    pub fn func_4() -> Vec<Vec<&Vec<Vec<u8>>>>;

    #[marine]
    pub fn func_5() -> Vec<Vec<Vec<&Vec<u8>>>>;

    #[marine]
    pub fn func_6() -> Vec<Vec<Vec<Vec<&u8>>>>;
}
