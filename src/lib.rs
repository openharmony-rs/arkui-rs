//! ArkUI Native
//!
//! Safe Rust abstractions to interact with ArkUI from native Rust code.
//! This crate is still under development and only provides safe abstractions for a subset
//! of the available C APIs. Currently, that is:
//!
//! - APIs of ArkUI to register gesture callbacks.
//!
//! The following features will hopefully be added soon:
//!
//! - drag and drop APIs of ArkUI
//! - animation callbacks of ArkUI
//! - UI capabilities such as UI component creation and destruction, tree node operations,
//!   attribute setting, and event listening.
//!
//! See the official upstream documentation on [ArkUI Native] for more information.
//!
//! [ArkUI Native]: https://docs.openharmony.cn/pages/v5.0/zh-cn/application-dev/reference/apis-arkui/_ark_u_i___native_module.md
//!
#![cfg(feature = "api-12")]

pub mod gestures;
