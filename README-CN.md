<div align="center">

# est

[English](README.md) | 简体中文

对 rust **标**准库 和 **T**okio 的 **扩**展

[官网](https://opensound.run) | [crates.io](https://crates.io/crates/est) | [docs.rs](https://docs.rs/est/latest/est) | [更新日志](CHANGELOG.md)

原始作者：[@czy-29](https://github.com/czy-29)

最新版本：[v0.10.0](https://github.com/opensound-org/est/releases/tag/v0.10.0)

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/opensound-org/est)
![Crates.io Total Downloads](https://img.shields.io/crates/d/est)
[![Crates.io Dependents](https://img.shields.io/crates/dependents/est)](https://crates.io/crates/est/reverse_dependencies)
![GitHub Repo stars](https://img.shields.io/github/stars/opensound-org/est)

![MSRV (version)](https://img.shields.io/crates/msrv/est/0.10.0?label=v0.10.0-msrv)
[![dependency status (version)](https://deps.rs/crate/est/0.10.0/status.svg?subject=v0.10.0-deps)](https://deps.rs/crate/est/0.10.0)

![MSRV (git)](https://img.shields.io/badge/git--msrv-1.85.0-blue)
[![dependency status (git)](https://deps.rs/repo/github/opensound-org/est/status.svg?subject=git-deps)](https://deps.rs/repo/github/opensound-org/est)

[![Static Badge](https://img.shields.io/badge/build_with-Rust_1.88.0-dca282)](https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/)

</div>

## 是什么
对 rust **标**准库 和 **T**okio 的 **扩**展。

部分条目如下：
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

请去 [docs.rs](https://docs.rs/est/latest/est) 页面查看更多详情。

## 为什么
主要是 [opensound-org](https://github.com/orgs/opensound-org/repositories) 组织的项目的开发过程中抽象出来的最底层的可复用组件。当然，其他人可能也会发觉这些东西有些用处。

## 怎么用
您可以：
```
cargo add est
```
或者在您的 `Cargo.toml` 中：
```toml
[dependencies]
est = "0.10"
```

## Star历史

[![Star History Chart](https://api.star-history.com/svg?repos=opensound-org/est&type=Date)](https://star-history.com/#opensound-org/est&Date)

# 许可证

本项目使用以下两种许可之一

 * Apache协议，2.0版本，([LICENSE-APACHE](LICENSE-APACHE) 或
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT协议 ([LICENSE-MIT](LICENSE-MIT) 或
   http://opensource.org/licenses/MIT)

由您选择。

## 贡献

[Github](https://github.com/opensound-org/est)是我们的[单一信源](https://en.wikipedia.org/wiki/Single_source_of_truth)，这里我们欢迎所有的issue和pull request。

我们另有两个[自动推送](.github/workflows/mirror.yml)的下游只读镜像：
- [Gitea](https://gitea.29bot.com/opensound-org/est)
- [Gitee](https://gitee.com/opensound-org/est)

由于它们是只读镜像，因此请不要在这两个平台上发起任何合并请求或pull request。

除非您另有明确说明，否则您有意提交的
包含在 `est` 中的任何贡献（如 Apache-2.0 许可证中所定义）均应
获得上述双重许可，无需任何附加条款或条件。
