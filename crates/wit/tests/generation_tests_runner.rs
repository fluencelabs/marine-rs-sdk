mod utils;

use utils::test_fce_token_streams;

#[test]
fn test() {
    test_fce_token_streams(
        "tests/generation_tests/export_functions/basic_types/fce.rs",
        "tests/generation_tests/export_functions/basic_types/expanded.rs",
    );

    test_fce_token_streams(
        "tests/generation_tests/records/call_parameters/fce.rs",
        "tests/generation_tests/records/call_parameters/expanded.rs",
    );
}
