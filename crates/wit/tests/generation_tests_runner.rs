mod utils;

use utils::test_fce_token_streams;

#[test]
fn exports_arrays() {
    assert!(test_fce_token_streams(
        "tests/generation_tests/exports/arrays/fce.rs",
        "tests/generation_tests/exports/arrays/expanded.rs",
    ));
}

#[test]
fn exports_basic_types() {
    assert!(test_fce_token_streams(
        "tests/generation_tests/exports/basic_types/fce.rs",
        "tests/generation_tests/exports/basic_types/expanded.rs",
    ));
}

#[test]
fn exports_refs() {
    assert!(test_fce_token_streams(
        "tests/generation_tests/exports/refs/fce.rs",
        "tests/generation_tests/exports/refs/expanded.rs",
    ));
}

#[test]
fn records_call_parameters() {
    assert!(test_fce_token_streams(
        "tests/generation_tests/records/call_parameters/fce.rs",
        "tests/generation_tests/records/call_parameters/expanded.rs",
    ));
}

#[test]
fn records_use_as_type() {
    assert!(test_fce_token_streams(
        "tests/generation_tests/records/use_as_type/fce.rs",
        "tests/generation_tests/records/use_as_type/expanded.rs",
    ));
}
