#![allow(improper_ctypes)]

use fluence::marine;

fn main() {}

#[marine]
struct StructWithPrivateFields {
    a: i32,
    b: usize,
}
