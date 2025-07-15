//! 堆記憶體管理

use std::ffi::c_uint;
use crate::JvmError;
use crate::runtime::JvmStack;

#[derive(Debug)]
pub struct Heap {
    size: usize,
}

// new
impl Heap {
    pub fn new(size: usize) -> Result<Self, JvmError> {
        Ok(Self { size })
    }
}