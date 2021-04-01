#![allow(improper_ctypes)]

use fluence::fce;

pub fn main() {}

#[fce]
fn test(_arg_1: Box<i32>) {}

#[fce]
fn test2(_arg_1: std::rc::Rc<i32>) {}

#[fce]
fn test3(_arg_1: std::collections::HashMap<i32, String>) {}

#[fce]
fn test4(_arg_1: i32) -> (i32, i32) {
    unimplemented!()
}

#[fce]
fn test5(_arg_1: i32) -> Box<i32> {
    unimplemented!()
}
