## Version 0.3.0 (2021-10-04)
[PR 61](https://github.com/fluencelabs/marine-rs-sdk/pull/61):

Implemented the first part of [Issue 57](https://github.com/fluencelabs/marine-rs-sdk/issues/57): `marine_test` now can take several named services in attributes, then define interface to the services in `marine_test_env`.

## Version 0.2.0 (2021-09-01)
[PR 54](https://github.com/fluencelabs/marine-rs-sdk/pull/54):
- previously test function accessed module interfaces through externally defined variables, now module interfaces are passed as arguments.
- introduced generated module `marine_test_env` which provides interface for generated structs and functions.
