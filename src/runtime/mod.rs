//! JVM 運行時數據區

mod heap;
mod method_area;
mod stack;
mod frame;

pub use heap::Heap;
pub use method_area::MethodArea;
pub use stack::JvmStack;
pub use frame::Frame;

use crate::Result;

/// JVM 運行時環境
pub struct JvmRuntime {
    /// 堆記憶體
    heap: Heap,

    /// 方法區
    method_area: MethodArea,

    /// 虛擬機棧
    stack: JvmStack,
}

impl JvmRuntime {
    /// 創建新的 JVM 運行時
    pub fn new() -> Result<Self> {
        Ok(Self {
            heap: Heap::new(1024 * 1024 * 64)?, // 64MB 初始 heap 大小
            method_area: MethodArea::new()?,
            stack: JvmStack::new(1024 * 1024)?, // 1MB stack 大小
        })
    }

    /// 啟動 JVM
    pub fn start(&mut self) -> Result<()> {
        log::info!("Starting JVM runtime...");
        // TODO: 實現啟動邏輯
        Ok(())
    }

    /// 關閉 JVM
    pub fn shutdown(&mut self) -> Result<()> {
        log::info!("Shutting down JVM runtime...");
        // TODO: 實現關閉邏輯
        Ok(())
    }
}