# Marine Rust SDK

[![crates.io version](https://img.shields.io/crates/v/marine-rs-sdk?color=green)](https://crates.io/crates/marine-rs-sdk)

This SDK empowers developers to create general-purpose Wasm modules and combine them in a multi-module setup with the help of [interface-types](https://github.com/WebAssembly/interface-types) and a [shared-nothing linking](https://training.linuxfoundation.org/blog/how-and-why-to-link-webassembly-modules/) scheme. The SDK provides all necessary macros and other features to make Wasm developing process as close as possible to the one with the "vanilla" Rust. Compiled modules are intended to run with the [Marine](https://github.com/fluencelabs/marine) runtime.


## Usage

The core component of the SDK is the `#[marine]` macro that should be used with export functions, external blocks, and structures. Let's consider a simple scenario with a module with one export function:
```rust
use marine_rs_sdk::marine;

#[marine]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}
```
This code imports the Marine SDK and wraps the greeting function with the `#[marine]` macro. Every function wrapped in such way will be exported from a compiled Wasm module.

To compile this code to Wasm, you need the Marine CLI tool. If you haven't installed the cli already, use `cargo install marine`:

```bash
marine build --release
```

Finally, you obtain a build manifest embedded into a Wasm binary and can interact with a module in the Marine REPL (mrepl). If you don't have it, install it with `cargo install mrepl`:

```bash
> marine info ./target/wasm32-wasi/release/greeting.wasm
it version:  0.23.1
sdk version: 0.7.0
authors:     <user-name>
version:     0.1.0
description:
repository:
build time:  2023-02-15 18:52:37.865550 +00:00 UTC

> mrepl --quiet
1> load greeting ./target/wasm32-wasi/release/greeting.wasm
module successfully loaded into App service
elapsed time: 52.153308ms

2> interface
Application service interface:
greeting:
  fn greeting(name: String) -> String

3> call greeting greeting "user"
result: String("Hi, user")
 elapsed time: 132.021µs
```

The complete guide of developing this simple module can be found [here](https://fluence.dev/docs/marine-book/quick-start/develop-a-single-module-service).


## SDK components

The SDK exports the following major components:
- `#[marine]` procedure macro that can be applied to
    - [function](https://fluence.dev/docs/marine-book/marine-rust-sdk/developing/export-functions) making it export from a module
    - [external block](https://fluence.dev/docs/marine-book/marine-rust-sdk/developing/import-functions) making it imported from a module in a Marine-suitable way
    - [structure](https://fluence.dev/docs/marine-book/marine-rust-sdk/developing/structures), making it usable as an argument of an export or an import function
- [call parameters interface](https://fluence.dev/docs/marine-book/marine-rust-sdk/developing/call-parameters) intended to provide a set of module start parameters
- [mounted binaries interface](https://fluence.dev/docs/marine-book/marine-rust-sdk/developing/mounted-binaries) that can be used to call a CLI tool
- [module_manifest](https://fluence.dev/docs/marine-book/marine-rust-sdk/developing/module-manifest) macro intended to embed some info into a compiled Wasm
- [logging stuff](https://fluence.dev/docs/marine-book/marine-rust-sdk/developing/logging) that allows adjusting a Marine logging mechanism


## Supported Rust types

At the moment, the `#[marine]` macro allows the following Rust types as an argument of export and import functions or the field of a structure:
- one of the following Rust basic types: `bool`, `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`
- strings `String`, `&str`
- a vector of elements of the above types
- a vector composed of vectors of the above type, where recursion is acceptable, e.g., the type `Vec<Vec<Vec<u8>>>` is permissible
- a reference of all of the above types
- a structure where all fields are of the basic Rust types
- a structure where all fields are of the above types or other structures build with such a way


## Documentation

- [Marine Book](https://fluence.dev/docs/marine-book/introduction)
- [Marine Examples](https://github.com/fluencelabs/examples/tree/main/marine-examples)
- [Quickstart](https://fluence.dev/docs/marine-book/quick-start/)

Also, check our [YouTube channel](https://www.youtube.com/@fluencelabs).


## Repository structure

- [**crates**](./crates)
    - [macro-testing-utils](./crates/macro-testing-utils) contains internal testing utils
    - [main](./crates/main) contains export_allocator, logger and results modules
    - [marine-macro](./crates/marine-macro) is the proc-macro crate for the `#[marine]` macro
    - [marine-macro-impl](./crates/marine-macro-impl) is the actual realization of the `#[marine]` macro
    - [timestamp-macro](./crates/timestamp-macro) is a macro to support timestamp embedding into a compiled Wasm binary
- [**src**](./src) contains call_parameters and mounted_binary modules along with reexporting all necessary for a user stuff


## Support

Please, [file an issue](https://github.com/fluencelabs/marine-rs-sdk/issues) if you find a bug. You can also contact us at [Discord](https://discord.com/invite/5qSnPZKh7u) or [Telegram](https://t.me/fluence_project). We will do our best to resolve the issue ASAP.


## Contributing

Any interested person is welcome to contribute to the project. Please, make sure you read and follow some basic [rules](./CONTRIBUTING.md).


## License

All software code is copyright (c) Fluence Labs, Inc. under the [AGPL v3.0](./LICENSE) license.
