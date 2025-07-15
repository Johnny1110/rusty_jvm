//! JVM Stack

use crate::JvmError;

pub struct JvmStack {
    max_depth : usize,
}

impl JvmStack {
    pub fn new(max_depth: usize) -> Result<Self, JvmError> {
        Ok(JvmStack {max_depth})
    }
}