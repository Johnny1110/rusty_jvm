//! Rusty JVM - A JVM implementation in Rust
//!
//! This is an educational project to understand JVM internals.

pub mod error;
pub mod classfile;
pub mod runtime;
pub mod classloader;
pub mod interpreter;
pub mod gc;

pub use error::{JvmError, Result};

pub use classfile::ClassFile;
pub use runtime::JvmRuntime;

/// JVM Version Info
pub const JVM_VERSION: &str = env!("CARGO_PKG_VERSION");

/// init log system
pub fn init_logger() {
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .init();
}