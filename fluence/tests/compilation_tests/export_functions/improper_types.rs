#![allow(improper_ctypes)]

use fluence::marine;

pub fn main() {}

#[marine]
fn test(_arg_1: Box<i32>) {}

#[marine]
fn test2(_arg_1: std::rc::Rc<i32>) {}

#[marine]
fn test3(_arg_1: std::collections::HashMap<i32, String>) {}

#[marine]
fn test4(_arg_1: i32) -> (i32, i32) {
    unimplemented!()
}

#[marine]
fn test5(_arg_1: i32) -> Box<i32> {
    unimplemented!()
}
