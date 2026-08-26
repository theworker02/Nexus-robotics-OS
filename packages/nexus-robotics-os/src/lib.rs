//! Official facade crate for Nexus Robotics OS.
//!
//! This package provides a stable, recognizable crates.io entry point while
//! preserving focused subcrates for applications and integration authors.

pub use nexus_core as core;
pub use nexus_runtime as runtime;

/// Release version of the facade and workspace package set.
pub const RELEASE: &str = env!("CARGO_PKG_VERSION");
