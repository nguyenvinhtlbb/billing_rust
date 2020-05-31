use serde_repr::{Deserialize_repr, Serialize_repr};

/// 调试类型
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum BillDebugType {
    /// 不显示调试信息
    NoDebug = 0,
    /// 常用debug
    Common = 1,
    /// 完整debug
    Full = 2,
}

impl Default for BillDebugType {
    fn default() -> Self {
        BillDebugType::NoDebug
    }
}
