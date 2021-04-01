#![allow(improper_ctypes)]

use fluence::fce;

pub fn main() {}

#[fce]
#[link(wasm_import_module = "arguments_passing_effector")]
extern "C" {
    #[fce]
    fn test(_arg_1: Box<i32>);

    #[fce]
    fn test2(_arg_1: std::rc::Rc<i32>);

    #[fce]
    fn test3(_arg_1: std::collections::HashMap<i32, String>);

    #[fce]
    fn test4(_arg_1: i32) -> (i32, i32);

    #[fce]
    fn test5(_arg_1: i32) -> Box<i32>;
}
