use crate::common::{BillingData, ResponseError};
use async_trait::async_trait;

/// billing包处理器
#[async_trait]
pub trait BillingHandler: Send + Sync {
    /// 获取处理类型
    fn get_type(&self) -> u8;
    /// 处理request包,返回response包
    async fn get_response(&mut self, request: &BillingData) -> Result<BillingData, ResponseError>;
}
