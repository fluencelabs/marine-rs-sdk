#[test]
fn marine_compilation_tests() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/compilation_tests/export_functions/array_inner_refs.rs");
    tests.pass("tests/compilation_tests/export_functions/arrays.rs");
    tests.pass("tests/compilation_tests/export_functions/ref_arrays.rs");
    tests.compile_fail("tests/compilation_tests/export_functions/inner_vec_refs.rs");
    tests.pass("tests/compilation_tests/export_functions/basic_types.rs");
    tests.pass("tests/compilation_tests/export_functions/ref_basic_types.rs");
    tests.compile_fail("tests/compilation_tests/export_functions/improper_types.rs");

    tests.compile_fail("tests/compilation_tests/import_functions/arrays_out_inner_refs.rs");
    tests.pass("tests/compilation_tests/import_functions/arrays.rs");
    tests.pass("tests/compilation_tests/import_functions/ref_arrays.rs");
    tests.pass("tests/compilation_tests/import_functions/basic_types.rs");
    tests.pass("tests/compilation_tests/import_functions/basic_ref_types.rs");
    tests.pass("tests/compilation_tests/import_functions/ref_basic_types.rs");
    tests.compile_fail("tests/compilation_tests/import_functions/improper_types.rs");

    tests.pass("tests/compilation_tests/records/basic_structs.rs");
    tests.pass("tests/compilation_tests/records/empty_struct.rs");
    tests.pass("tests/compilation_tests/records/struct_with_private_fields.rs");
    tests.compile_fail("tests/compilation_tests/records/struct_with_improper_types.rs");
    tests.compile_fail("tests/compilation_tests/records/unnamed_structs.rs");
}
