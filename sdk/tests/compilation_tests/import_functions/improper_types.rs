#![allow(improper_ctypes)]

use marine_rs_sdk::marine;

pub fn main() {}

#[marine]
#[link(wasm_import_module = "arguments_passing_effector")]
extern "C" {
    #[marine]
    fn test(_arg_1: Box<i32>);

    #[marine]
    fn test2(_arg_1: std::rc::Rc<i32>);

    #[marine]
    fn test3(_arg_1: std::collections::HashMap<i32, String>);

    #[marine]
    fn test4(_arg_1: i32) -> (i32, i32);

    #[marine]
    fn test5(_arg_1: i32) -> Box<i32>;
}
