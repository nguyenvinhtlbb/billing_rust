use crate::common::{BillingData, BillingHandler, ResponseError};
use async_trait::async_trait;

pub struct PingHandler;

#[async_trait]
impl BillingHandler for PingHandler {
    fn get_type(&self) -> u8 {
        0xA1
    }

    async fn get_response(&mut self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let mut response: BillingData = request.into();
        response.op_data.extend(&[0x01, 0x00]);
        Ok(response)
    }
}
