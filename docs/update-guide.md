# Marine Rust SDK update guide

## Marine Rust SDK repo components

Here is the list of main components crucial from the update point of view:

[**Marine Rust SDK**](./) - the interface crate that will be used by users
[**Marine Rust SDK main**](./crates/main) - contains all export functions used by IT interpreter, as well as logger

## Marine Rust SDK update policy

### Versioning
All the crates in this repo have the same version, and all repository-local dependencies are strict (denoted by `=x.y.z` notation). 

### Coupling with AquaVM

This crate uses `#[no_mangle]` exports in `marine-rs-sdk-main`, and in `marine-rs-sdk` crate by transitivity. This means that it is impossible to use two different versions of this crate when compiling single binary, regardless of versions. Also there is a circular cross-repository dependency with [**AquaVM**](https://github.com/fluencelabs/aquavm):
[**polyplets**](https://github.com/fluencelabs/aquavm/tree/master/crates/air-lib/polyplets) depends on `marine-rs-sdk-main`, while `marine-rs-sdk` depends on `polyplets`. AquaVM repo also uses `marine-rs-sdk` in several crates.

The conclusion is that there is no way to just update minor or patch version of this repo. It also requires updating `polyplets` to a version that uses `marine-rs-sdk-main` with the same major and minor versions as `marine-rs-sdk`.

The following update process should ensure that there is no semver-broken version is used:
1. release new *minor* version `marine-rs-sdk@0.x.0`and so on
2. update AquaVM to `marine-rs-sdk@0.x.0` and so on, bump *minor* version for `polyplets` to `0.y.0`
3. release `marine-rs-sdk@0.x.1` that uses `polyplets@0.y.0`
4. update AquaVM to use `marine-rs-sdk@0.x.1` and so on, bump *patch* version on `poluplets` to `0.y.1`
5. yank `marine-rs-sdk@0.x.0` and so on

This process **WILL** break build of the AquaVM between the steps 2-4, so the whole process should be completed without delays. `mairne-rs-sdk@0.x.0` will contain different minor versions of `marine-rs-sdk-main` so it should never be used -- therefore it is yanked.

