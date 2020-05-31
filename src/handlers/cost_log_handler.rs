use crate::common::{BillingData, BillingHandler, ResponseError};
use async_trait::async_trait;

pub struct CostLogHandler;

#[async_trait]
impl BillingHandler for CostLogHandler {
    fn get_type(&self) -> u8 {
        0xC5
    }

    async fn get_response(&mut self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let request_op_data = request.op_data.as_slice();
        let op_data = &request_op_data[..21];
        let mut response: BillingData = request.into();
        response.op_data.extend_from_slice(op_data);
        response.op_data.push(0x01);
        Ok(response)
    }
}
