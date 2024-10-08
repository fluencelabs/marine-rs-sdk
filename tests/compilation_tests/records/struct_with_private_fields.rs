#![allow(improper_ctypes)]

use marine_rs_sdk::marine;

fn main() {}

#[marine]
struct StructWithPrivateFields {
    a: i32,
    b: usize,
}

#[marine]
fn export_func(_field: StructWithPrivateFields) { }

#[marine]
#[module_import("record_module")]
extern "C" {
    fn import_func(arg: &StructWithPrivateFields) -> StructWithPrivateFields;
}
