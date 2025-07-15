//! Class 檔案格式相關模組

mod parser;
mod constant_pool;
mod attributes;

pub use parser::ClassFileParser;
pub use constant_pool::{ConstantPool, ConstantPoolEntry};
pub use attributes::Attribute;

use crate::Result;

/// Class 檔案結構
#[derive(Debug)]
pub struct ClassFile {
    /// 魔數 (0xCAFEBABE)
    pub magic: u32,

    /// 次版本號
    pub minor_version: u16,

    /// 主版本號
    pub major_version: u16,

    /// 常量池
    pub constant_pool: ConstantPool,

    /// 訪問標誌
    pub access_flags: u16,

    /// 當前類索引
    pub this_class: u16,

    /// 父類索引
    pub super_class: u16,

    /// 介面表
    pub interfaces: Vec<u16>,

    /// 字段表
    pub fields: Vec<FieldInfo>,

    /// 方法表
    pub methods: Vec<MethodInfo>,

    /// 屬性表
    pub attributes: Vec<Attribute>,
}

/// 字段資訊
#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}

/// 方法資訊
#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    /// 從檔案路徑載入 class 檔案
    pub fn from_path(path: &std::path::Path) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    /// 從位元組陣列解析 class 檔案
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        ClassFileParser::parse(bytes)
    }
}