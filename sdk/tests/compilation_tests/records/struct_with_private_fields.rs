#![allow(improper_ctypes)]

use marine_rs_sdk::marine;

fn main() {}

#[marine]
struct StructWithPrivateFields {
    a: i32,
    b: usize,
}
