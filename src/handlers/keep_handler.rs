use crate::common::{BillingData, BillingHandler, ResponseError};
use crate::services::read_buffer_slice;
use async_trait::async_trait;
use std::str;

pub struct KeepHandler;

#[async_trait]
impl BillingHandler for KeepHandler {
    fn get_type(&self) -> u8 {
        0xA6
    }

    async fn get_response(&self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let offset = 0;
        let request_op_data = request.op_data.as_slice();
        //用户名
        let (username, offset) = read_buffer_slice(request_op_data, offset);
        let username_str = str::from_utf8(username).unwrap();
        let user_level = {
            let mut result_value = 0u16;
            result_value += (request_op_data[offset] << 8) as u16;
            result_value += request_op_data[offset + 1] as u16;
            result_value
        };
        println!("keep: user [{}] level {}", username_str, user_level);
        //todo 更新在线状态
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend_from_slice(username);
        response.op_data.push(0x01);
        Ok(response)
    }
}
