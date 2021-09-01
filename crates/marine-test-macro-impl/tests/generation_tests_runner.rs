/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
