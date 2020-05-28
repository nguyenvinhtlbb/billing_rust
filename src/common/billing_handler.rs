use crate::common::{BillingData, ResponseError};
use async_trait::async_trait;

/// billing包处理器
#[async_trait]
pub trait BillingHandler: Send + Sync {
    /// 获取处理类型
    fn get_type(&self) -> u8;
    /// 获取请求包的响应包
    async fn get_response(&self, request: &BillingData) -> Result<BillingData, ResponseError>;
}
