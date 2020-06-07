// Copyright (c) 2019 Heath Stewart.
// Licensed under the MIT License.

#![allow(dead_code)]

mod secrets;

#[cfg(feature = "secrets")]
pub use secrets::SecretClient;
