mod utils;

use utils::test_marine_test_token_streams;

#[test]
fn test_empty_func() {
    assert!(test_marine_test_token_streams(
        "tests/generation_tests/empty_func/marine_test.rs",
        "tests/generation_tests/empty_func/expanded.rs",
        "Config.toml",
        "artifacts"
    ));
}

#[test]
fn test_mounted_binary() {
    assert!(test_marine_test_token_streams(
        "tests/generation_tests/mounted_binary/marine_test.rs",
        "tests/generation_tests/mounted_binary/expanded.rs",
        "Config.toml",
        "artifacts"
    ));
}

#[test]
fn test_multiple_modules() {
    assert!(test_marine_test_token_streams(
        "tests/generation_tests/multiple_modules/marine_test.rs",
        "tests/generation_tests/multiple_modules/expanded.rs",
        "Config.toml",
        "artifacts"
    ));
}
