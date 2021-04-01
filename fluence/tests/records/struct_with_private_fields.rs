#![allow(improper_ctypes)]

use fluence::fce;

fn main() {}

#[fce]
struct StructWithPrivateFields {
    a: i32,
    b: usize,
}
