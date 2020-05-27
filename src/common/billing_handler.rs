use crate::common::{BillingData, ResponseError};
use async_trait::async_trait;

#[async_trait]
pub trait BillingHandler: Send + Sync {
    fn get_type(&self) -> u8;
    async fn get_response(&self, request: &BillingData) -> Result<BillingData, ResponseError>;
}
