# Changelog

## [0.11.0](https://github.com/fluencelabs/marine-rs-sdk/compare/marine-rs-sdk-v0.10.3...marine-rs-sdk-v0.11.0) (2024-02-05)


### ⚠ BREAKING CHANGES

* **ABI, call-parameters:** add worker_id field into CallParameters ([#151](https://github.com/fluencelabs/marine-rs-sdk/issues/151))

### Features

* **ABI, call-parameters:** add worker_id field into CallParameters ([#151](https://github.com/fluencelabs/marine-rs-sdk/issues/151)) ([3bdf0c2](https://github.com/fluencelabs/marine-rs-sdk/commit/3bdf0c242aa1da62be319f2131b9c14e396d23f9))

## [0.10.3](https://github.com/fluencelabs/marine-rs-sdk/compare/marine-rs-sdk-v0.10.2...marine-rs-sdk-v0.10.3) (2023-12-27)


### Features

* **call-parameters:** optional rkyv support ([#148](https://github.com/fluencelabs/marine-rs-sdk/issues/148)) ([22863bc](https://github.com/fluencelabs/marine-rs-sdk/commit/22863bc94f620fcd9fee4dc1476a71248cc94963))

## [0.10.2](https://github.com/fluencelabs/marine-rs-sdk/compare/marine-rs-sdk-v0.10.1...marine-rs-sdk-v0.10.2) (2023-12-12)


### Bug Fixes

* **ABI:** use correct empty string/vector representation  ([#146](https://github.com/fluencelabs/marine-rs-sdk/issues/146)) ([d3cade9](https://github.com/fluencelabs/marine-rs-sdk/commit/d3cade98f8555cb31907d890a91c87bb53ee9c18))

## [0.10.1](https://github.com/fluencelabs/marine-rs-sdk/compare/marine-rs-sdk-v0.10.0...marine-rs-sdk-v0.10.1) (2023-10-24)


### Features

* **deps:** update rust crate chrono to 0.4.31 ([#119](https://github.com/fluencelabs/marine-rs-sdk/issues/119)) ([69d04d1](https://github.com/fluencelabs/marine-rs-sdk/commit/69d04d1cbd80068aa5bb4cb5aeb7d09d0349c4f3))
* **deps:** update rust crate log to 0.4.20 ([#118](https://github.com/fluencelabs/marine-rs-sdk/issues/118)) ([b08bcf1](https://github.com/fluencelabs/marine-rs-sdk/commit/b08bcf1bd183667e636f3033cf51d6fd6328a7b8))
* **deps:** update rust crate pretty_assertions to 1.4.0 ([#137](https://github.com/fluencelabs/marine-rs-sdk/issues/137)) ([00f745f](https://github.com/fluencelabs/marine-rs-sdk/commit/00f745fa157a0105f4c32875079ed34c040c221f))
* **deps:** update rust crate proc-macro2 to 1.0.69 ([#111](https://github.com/fluencelabs/marine-rs-sdk/issues/111)) ([7cd6d86](https://github.com/fluencelabs/marine-rs-sdk/commit/7cd6d867830a44763b26470a0ac37f8f61e8d3e7))
* **deps:** update rust crate quote to 1.0.33 ([0e4cb4b](https://github.com/fluencelabs/marine-rs-sdk/commit/0e4cb4b1f2742095d1f42ee08b2110553100fb8b))
* **deps:** update rust crate serde to 1.0.189 ([#136](https://github.com/fluencelabs/marine-rs-sdk/issues/136)) ([9827098](https://github.com/fluencelabs/marine-rs-sdk/commit/9827098735b0e01288acc844ebceae35ea5ef96b))
* **deps:** update rust crate serde_json to 1.0.107 ([#117](https://github.com/fluencelabs/marine-rs-sdk/issues/117)) ([513a80a](https://github.com/fluencelabs/marine-rs-sdk/commit/513a80aa153cb61a0da133aba734b7a5ba700fab))

## [0.10.0](https://github.com/fluencelabs/marine-rs-sdk/compare/marine-rs-sdk-v0.9.0...marine-rs-sdk-v0.10.0) (2023-09-13)


### ⚠ BREAKING CHANGES

* hide marine ABI exports under `marine-abi` feature ([#129](https://github.com/fluencelabs/marine-rs-sdk/issues/129))

### Features

* hide marine ABI exports under `marine-abi` feature ([#129](https://github.com/fluencelabs/marine-rs-sdk/issues/129)) ([7059e84](https://github.com/fluencelabs/marine-rs-sdk/commit/7059e84635819925b7f84e5b61260037f2ceb265))

## [0.9.0](https://github.com/fluencelabs/marine-rs-sdk/compare/marine-rs-sdk-v0.8.1...marine-rs-sdk-v0.9.0) (2023-08-16)


### ⚠ BREAKING CHANGES

* move SecurityTetraplets from popyplets directly to marine-rs-sdk crate ([#127](https://github.com/fluencelabs/marine-rs-sdk/issues/127))

### Features

* move SecurityTetraplets from popyplets directly to marine-rs-sdk crate ([#127](https://github.com/fluencelabs/marine-rs-sdk/issues/127)) ([add9b91](https://github.com/fluencelabs/marine-rs-sdk/commit/add9b919edcefa06b67975170ea149d148911073))

## [0.8.1](https://github.com/fluencelabs/marine-rs-sdk/compare/marine-rs-sdk-v0.8.0...marine-rs-sdk-v0.8.1) (2023-08-09)


### Bug Fixes

* update polyplets to 0.4.0 as part of minor version update process ([#125](https://github.com/fluencelabs/marine-rs-sdk/issues/125)) ([88897ba](https://github.com/fluencelabs/marine-rs-sdk/commit/88897bac8b32311c2de2863afae3436a343e2b20))

## [0.8.0](https://github.com/fluencelabs/marine-rs-sdk/compare/marine-rs-sdk-v0.7.1...marine-rs-sdk-v0.8.0) (2023-07-24)


### ⚠ BREAKING CHANGES

* allow field attibutes ([#121](https://github.com/fluencelabs/marine-rs-sdk/issues/121))

### Features

* allow field attibutes ([#121](https://github.com/fluencelabs/marine-rs-sdk/issues/121)) ([7a39cd3](https://github.com/fluencelabs/marine-rs-sdk/commit/7a39cd35ada38a8c38e0b1643e88d0f1601a5030))
