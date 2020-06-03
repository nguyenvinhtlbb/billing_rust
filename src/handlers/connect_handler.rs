use crate::common::{BillingData, BillingHandler, ResponseError};
use async_trait::async_trait;

pub struct ConnectHandler;

#[async_trait]
impl BillingHandler for ConnectHandler {
    fn get_type(&self) -> u8 {
        0xA0
    }

    async fn get_response(&mut self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let mut response: BillingData = request.into();
        response.op_data.extend(&[0x20, 0x00]);
        Ok(response)
    }
}
