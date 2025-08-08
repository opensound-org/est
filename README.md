<div align="center">

# est

English | [简体中文](README-CN.md)

**E**xtensions for the rust **S**tandard library and **T**okio

[Website](https://opensound.run) | [crates.io](https://crates.io/crates/est) | [docs.rs](https://docs.rs/est/latest/est) | [Changelog](CHANGELOG.md)

Original Author: [@czy-29](https://github.com/czy-29)

Latest version: [v0.10.1](https://github.com/opensound-org/est/releases/tag/v0.10.1)

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/opensound-org/est)
![Crates.io Total Downloads](https://img.shields.io/crates/d/est)
[![Crates.io Dependents](https://img.shields.io/crates/dependents/est)](https://crates.io/crates/est/reverse_dependencies)
![GitHub Repo stars](https://img.shields.io/github/stars/opensound-org/est)

![MSRV (version)](https://img.shields.io/crates/msrv/est/0.10.1?label=v0.10.1-msrv)
[![dependency status (version)](https://deps.rs/crate/est/0.10.1/status.svg?subject=v0.10.1-deps)](https://deps.rs/crate/est/0.10.1)

![MSRV (git)](https://img.shields.io/badge/git--msrv-1.85.0-blue)
[![dependency status (git)](https://deps.rs/repo/github/opensound-org/est/status.svg?subject=git-deps)](https://deps.rs/repo/github/opensound-org/est)

[![Static Badge](https://img.shields.io/badge/build_with-Rust_1.89.0-dca282)](https://blog.rust-lang.org/2025/08/07/Rust-1.89.0/)

</div>

## What
**E**xtensions for the rust **S**tandard library and **T**okio.

Some of the items are as follows:
- [`AnyRes`](https://docs.rs/est/latest/est/result/type.AnyRes.html)
- [`collections::MapExt::replace_key()`](https://docs.rs/est/latest/est/collections/trait.MapExt.html#tymethod.replace_key)
- [`future::FutureExt::with_cancel_signal()`](https://docs.rs/est/latest/est/future/trait.FutureExt.html#tymethod.with_cancel_signal)
- [`future::IntoFutureWithArgs`](https://docs.rs/est/latest/est/future/trait.IntoFutureWithArgs.html)
- [`process::Command`](https://docs.rs/est/latest/est/process/enum.Command.html)
- [`slice::SliceExt::has_dup()`](https://docs.rs/est/latest/est/slice/trait.SliceExt.html#tymethod.has_dup)
- [`sync::once`](https://docs.rs/est/latest/est/sync/once/index.html)
- [`task::task_tracker::CloseAndWait::close_and_wait()`](https://docs.rs/est/latest/est/task/task_tracker/trait.CloseAndWait.html#tymethod.close_and_wait)
- [`task::GracefulTask`](https://docs.rs/est/latest/est/task/graceful/struct.GracefulTask.html)
- [`task::TaskId`](https://docs.rs/est/latest/est/task/struct.TaskId.html)
- [`thread::ThreadId`](https://docs.rs/est/latest/est/thread/struct.ThreadId.html)

Please visit the [docs.rs](https://docs.rs/est/latest/est) page for more details.

## Why
Mainly the lowest level reusable components abstracted during the development process of [opensound-org](https://github.com/orgs/opensound-org/repositories) projects. Of course, others may also find these things useful.

## How
You can:
```
cargo add est
```
Or in your `Cargo.toml`:
```toml
[dependencies]
est = "0.10"
```

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=opensound-org/est&type=Date)](https://star-history.com/#opensound-org/est&Date)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

[Github](https://github.com/opensound-org/est) is our [single source of truth](https://en.wikipedia.org/wiki/Single_source_of_truth), where we welcome all issues and pull requests.

We also have two downstream read-only mirrors that are [automatically pushed](.github/workflows/mirror.yml):
- [Gitea](https://gitea.29bot.com/opensound-org/est)
- [Gitee](https://gitee.com/opensound-org/est)

As they are read-only mirrors, please do not initiate any merge or pull requests on these two platforms.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `est` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
