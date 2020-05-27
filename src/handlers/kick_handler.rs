use crate::common::{BillingData, BillingHandler, ResponseError};
use async_trait::async_trait;

pub struct KickHandler;

#[async_trait]
impl BillingHandler for KickHandler {
    fn get_type(&self) -> u8 {
        0xA9
    }

    async fn get_response(&self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let mut response: BillingData = request.into();
        response.op_data.push(0x01);
        Ok(response)
    }
}
