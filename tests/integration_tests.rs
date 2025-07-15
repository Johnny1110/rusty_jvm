//! 整合測試

use rusty_jvm::Result;

#[test]
fn test_jvm_creation() -> Result<()> {
    let runtime = rusty_jvm::runtime::JvmRuntime::new()?;
    // 基本的創建測試
    Ok(())
}

#[test]
fn test_error_handling() {
    use rusty_jvm::JvmError;

    // 測試錯誤類型
    let err = JvmError::ClassNotFound("java.lang.Object".to_string());
    assert_eq!(err.to_string(), "Class not found: java.lang.Object");

    let err = JvmError::InvalidMagicNumber(0x12345678);
    assert_eq!(err.to_string(), "Invalid magic number: expected 0xCAFEBABE, got 0x12345678");
}