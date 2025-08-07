# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

### Changed

- (safety): Replace `unwrap` with `expect` to enhance safety and readability ([](https://github.com/opensound-org/est/commit/))

## 0.10.0 - 2025-07-08

### Added

- (slice): Introduce `SliceExt` trait with `has_dup` method ([fa37a1](https://github.com/opensound-org/est/commit/fa37a1110144a9813db2dd45cbb7add3f8b29e1b))

### Docs

- (README): Update `git-msrv` badge ([a9dc28](https://github.com/opensound-org/est/commit/a9dc2807d9cdb8d1c22b74c6c86e3b07448820b2))

### Deps

- Upgrade `indexmap` to `2.10.0` ([818403](https://github.com/opensound-org/est/commit/818403c192681567b9485dec910c8a7f4f769431))
- Upgrade `tokio` to `1.46.1` ([be1237](https://github.com/opensound-org/est/commit/be123737c58f356184f06a725895f1542a8a5653))
- (README): Update `Rust` to `1.88.0` ([067a7e](https://github.com/opensound-org/est/commit/067a7ec5c108b87444e2ef679e46244925008d3d))

## 0.9.1 - 2025-05-25

### Fixed

- Fix inconsistent dependency versions ([99cb9b](https://github.com/opensound-org/est/commit/99cb9bd903ebab7361636c6c422c87243f357e2a))

### Docs

- (README): Add `DeepWiki` badge ([7a37b4](https://github.com/opensound-org/est/commit/7a37b4416990da1c8ad2dc0e61d9c5455049b541))

### Deps

- Upgrade `tokio` to `1.45.0` ([0850b2](https://github.com/opensound-org/est/commit/0850b27eaa91e5807d0705fff4d01d11e79f5529))
- Upgrade `tokio` to `1.45.1` ([efcdc6](https://github.com/opensound-org/est/commit/efcdc68747aef82809dafd5ca8b7f8b41945c2f8))
- (README): Update `Rust` to `1.87.0` ([547b29](https://github.com/opensound-org/est/commit/547b298208998942e2b8bc54f8ef21df5db48b7d))

## 0.9.0 - 2025-04-25

### Added

- (collections): Re-exports `indexmap::IndexMap` ([5155f2](https://github.com/opensound-org/est/commit/5155f29a71a740e2c4fbfa4939a3bb595cbc2486))
- (future): Add tests on `async closure` to `IntoFutureWithArgs` ([17107b](https://github.com/opensound-org/est/commit/17107b2b8ab19049b27d8427ed0d661d07875c2c))
- (task): Introduce `GracefulTask` and surrounding facilities ([10bb7d](https://github.com/opensound-org/est/commit/10bb7d4a7b3593f60fc3187ad2899d01d7bb56f1))
- (task::graceful): Add some mock interfaces for test driven design ([8fda8b](https://github.com/opensound-org/est/commit/8fda8b33f2ac6bd548eadb7a7c62be188975a7ad))
- (task::graceful): Add some unit tests ([9dd80f](https://github.com/opensound-org/est/commit/9dd80f87a02047e4e888939635c0fadac1e5c561))
- (task::graceful): Add some unit tests ([45be0f](https://github.com/opensound-org/est/commit/45be0fd002ad976b4880057741cebfd41a126a72))
- (task::graceful): Add some unit tests ([f4863a](https://github.com/opensound-org/est/commit/f4863a8b0f12f11127b0d3f6e475267ead8e6a2f))
- Introduce feature flag mechanism ([16fea9](https://github.com/opensound-org/est/commit/16fea93658814ccb325016868c20dc91a269dd52))

### Changed

- (task::graceful): Simplify some unit tests ([d145bd](https://github.com/opensound-org/est/commit/d145bdb157d444d55911565abe5660f5b2a12275))
- (task::graceful): Update some unit tests ([8f1f61](https://github.com/opensound-org/est/commit/8f1f616a4a5f56a746bf50f13208ed3511208d48))
- (task::graceful): Use `std::sync::Mutex` instead of `tokio::sync::Mutex` to reduce some dependence on `tokio` ([e2ef3f](https://github.com/opensound-org/est/commit/e2ef3fcafabd863f01c4e883d875757c69d9eec0))

### Docs

- (sync::once): Add some documentation to `OnceWaiter::triggered` ([8bfde4](https://github.com/opensound-org/est/commit/8bfde4179e717dbf7f8ddf0768ab1bf63f07016a))
- (task): Fix missing punctuation ([0283bf](https://github.com/opensound-org/est/commit/0283bf398f97531b0467fcf0e7963cce5633ece0))
- (task::graceful): Add full document ([c967a8](https://github.com/opensound-org/est/commit/c967a83b39544d0bd5d1cc88a9ce46e3a3394cce))
- (task::GracefulTask): Add some additional docs ([2802ed](https://github.com/opensound-org/est/commit/2802ede69f77f2c4b764b958b7c66bbcea505ad0))

### Deps

- (README): Update `Rust` to `1.86.0` ([bc58e4](https://github.com/opensound-org/est/commit/bc58e4c02ca08a7947151751b041727c2de46c4a))
- Upgrade `indexmap` to `2.9.0` ([e4df6d](https://github.com/opensound-org/est/commit/e4df6d87dd1a158d0e6586f76366f8d733eae306))
- Upgrade `tokio` to `1.44.2` ([6907bb](https://github.com/opensound-org/est/commit/6907bbe09828d114d82249a0d61c9b0f72ca6fa5))
- Upgrade `anyhow` to `1.0.98` ([fd2c04](https://github.com/opensound-org/est/commit/fd2c04b32419344f65ef30efdc7435afffc2276a))
- Upgrade `ron` to `0.10.1` ([d4072d](https://github.com/opensound-org/est/commit/d4072dd08bb23a03f3f20acd6b2d246b54dd399c))
- Upgrade `tokio-util` to `0.7.15` ([bcadd5](https://github.com/opensound-org/est/commit/bcadd5801620b2dfc2c3f7b874f4819fac50f3a9))

## 0.8.1 - 2025-03-28

### Docs

- Optimize the reference path of documents ([86ee67](https://github.com/opensound-org/est/commit/86ee6737566a7c7cef7e95f3838626a2fc1aef9c))

## 0.8.0 - 2025-03-28

### Added

- (future): Introduce `FutureExt::with_cancel_signal()` and `WithCancelSignal` Future ([43c398](https://github.com/opensound-org/est/commit/43c398dc930302085320802e4dd436ec30cdf4b5))
- (future): Introduce `IntoFutureWithArgs` trait ([e5f645](https://github.com/opensound-org/est/commit/e5f645c2a27ed815d6aafb15a26bee217f9b9f71))
- (task): Re-exports `tokio_util::task::TaskTracker` ([23cdaa](https://github.com/opensound-org/est/commit/23cdaa5356d30060e83bda42918ac268bbac28b4))
- (process): Introduce `Command` enum ([02076b](https://github.com/opensound-org/est/commit/02076b31a1e46cdaa7e34b42bec69a8e48b722d1))
- Re-exports `tokio` ([fcbbe3](https://github.com/opensound-org/est/commit/fcbbe33897b21bc56516cb131a9203c61c5c0015))

### Refactor

- `Future` is already a `prelude` in `2024 Edition`, so no `use` is needed. ([65cb73](https://github.com/opensound-org/est/commit/65cb738839c4488b10f47e04b32a265cdb0d1639))
- (task): Move `CloseAndWait` trait to the `task_tracker` child mod ([578d3f](https://github.com/opensound-org/est/commit/578d3f32921ef72c6c3ed2fb712ea9fbe5401edc))

### Infra

- (workflows): Temporarily remove the mirror to `gitea` ([0d005e](https://github.com/opensound-org/est/commit/0d005eebf7a09ffd8024dc767d588753745b388d))

### Docs

- (README): Modify the path of `CloseAndWait` trait ([950290](https://github.com/opensound-org/est/commit/950290f8f4f9a063efd6ca7142c58fb6cf79465a))

### Deps

- (README): Update `Rust` to `1.84.1` ([e681d4](https://github.com/opensound-org/est/commit/e681d4e9637e9fc632891e11883884280b6be5d9))
- Upgrade `derive_more` to `2.0.1` ([969197](https://github.com/opensound-org/est/commit/96919795c4cc1deb30345453fb93ab8cbb6b0855))
- Upgrade `anyhow` to `1.0.97` ([3166e2](https://github.com/opensound-org/est/commit/3166e24f71b2a5fe502d3153aaafff6c18062acc))
- Upgrade `serde` to `1.0.218` ([0ed985](https://github.com/opensound-org/est/commit/0ed9854841f25d5cb6cc7dd4902c5bdbe0c58166))
- Upgrade `thiserror` to `2.0.12` ([0dfc8b](https://github.com/opensound-org/est/commit/0dfc8b8200632b4455afe312d2fdb266977f2311))
- Upgrade `tokio` to `1.44.0` ([378c5e](https://github.com/opensound-org/est/commit/378c5e3bc623214fe2a9e8bdc731108d16fc9bc4))
- Update `Rust` to version `1.85.0` and `Edition 2024`. ([b5caf6](https://github.com/opensound-org/est/commit/b5caf67380ecd046cac6b1a6acdfaa6593384e91))
- (README): Update `Rust` to `1.85.1` ([a99b31](https://github.com/opensound-org/est/commit/a99b3112948d3119e188f8cf0463da43378199d7))
- Upgrade `indexmap` to `2.8.0` ([b5f8d4](https://github.com/opensound-org/est/commit/b5f8d4ee163ea42d73f0819d2455dc9673e2361d))
- Upgrade `ron` to `0.9.0` ([681872](https://github.com/opensound-org/est/commit/6818724584d000432b5e4607f1042201d4908df2))
- Upgrade `serde` to `1.0.219` ([b83512](https://github.com/opensound-org/est/commit/b835120e3c13ddc5ee05172065b4b669f7625037))
- Upgrade `tokio` to `1.44.1` ([6a3e3b](https://github.com/opensound-org/est/commit/6a3e3bc9817324cc280afee93623fc3ead5eb0af))
- Upgrade `tokio-util` to `0.7.14` ([e30b5f](https://github.com/opensound-org/est/commit/e30b5f0618461ace77e3629ef9c01ee472beb61d))
