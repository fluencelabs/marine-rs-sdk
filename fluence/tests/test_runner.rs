#[test]
fn test() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/export_functions/array_inner_refs.rs");
    tests.pass("tests/export_functions/arrays.rs");
    tests.pass("tests/export_functions/ref_arrays.rs");
    tests.pass("tests/export_functions/basic_types.rs");
    tests.pass("tests/export_functions/ref_basic_types.rs");
    tests.compile_fail("tests/export_functions/improper_types.rs");

    tests.compile_fail("tests/import_functions/arrays_out_inner_refs.rs");
    tests.pass("tests/import_functions/arrays.rs");
    tests.pass("tests/import_functions/ref_arrays.rs");
    tests.pass("tests/import_functions/basic_types.rs");
    tests.pass("tests/import_functions/ref_basic_types.rs");
    tests.compile_fail("tests/import_functions/improper_types.rs");

    tests.pass("tests/records/basic_structs.rs");
    tests.pass("tests/records/empty_struct.rs");
    tests.compile_fail("tests/records/struct_with_improper_types.rs");
    tests.compile_fail("tests/records/struct_with_private_fields.rs");
    tests.compile_fail("tests/records/unnamed_structs.rs");
}
