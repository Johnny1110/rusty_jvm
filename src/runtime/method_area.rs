//! 方法區

use crate::JvmError;

pub struct MethodArea {

}

impl MethodArea {
    pub fn new() -> Result<Self, JvmError> {
        Ok(MethodArea {})
    }
}