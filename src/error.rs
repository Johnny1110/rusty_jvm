//! JVM Error Handle System
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JvmError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Class file parse error: {0}")]
    ClassFileParse(String),

    #[error("Invalid magic number: expected 0xCAFEBABE, got 0x{0:08X}")]
    InvalidMagicNumber(u32),

    #[error("Unsupported class file version: {major}.{minor}")]
    UnsupportedVersion { major: u16, minor: u16 },

    #[error("Invalid constant pool index: {0}")]
    InvalidConstantPoolIndex(u16),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Class not found: {0}")]
    ClassNotFound(String),

    #[error("Method not found: {class}.{method}{descriptor}")]
    MethodNotFound {
        class: String,
        method: String,
        descriptor: String,
    },

    #[error("Out of memory")]
    OutOfMemory,

    #[error("Stack overflow")]
    StackOverflow,
}

pub type Result<T> = std::result::Result<T, JvmError>;