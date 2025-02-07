// Copyright 2022 Oxide Computer Company

//! Progenitor is a Rust crate for generating opinionated clients from API
//! descriptions specified in the OpenAPI 3.0.x format. It makes use of Rust
//! futures for async API calls and `Streams` for paginated interfaces.
//!
//! It generates a type called `Client` with methods that correspond to the
//! operations specified in the OpenAPI document.
//!
//! For details see the [repo
//! README](https://github.com/oxidecomputer/progenitor/README.md)

pub use progenitor_client;
pub use progenitor_impl::Error;
pub use progenitor_impl::Generator;
pub use progenitor_macro::generate_api;
