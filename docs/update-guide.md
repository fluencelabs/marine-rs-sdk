# Marine Rust SDK update guide

## Marine Rust SDK repo components

Here is the list of main components crucial from the update point of view:

* [**Marine Rust SDK**](./) - the interface crate that will be used by users
* [**Marine Rust SDK main**](./crates/main) - contains all export functions used by IT interpreter, as well as logger

## Marine Rust SDK update policy

### Versioning

All the crates in this repo have the same version, and all repository-local dependencies are strict (denoted by `=x.y.z` notation). 

## Important notes

The `marine-rs-sdk-main` crate uses `#[no_mangle]` exports, and `marine-rs-sdk` crate too - by transitivity. That's why it is impossible to use two different versions of this `marine-rs-sdk-main` when compiling a single binary, regardless of their versions.
