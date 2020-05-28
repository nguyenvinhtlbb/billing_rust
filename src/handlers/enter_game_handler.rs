use crate::common::{BillingData, BillingHandler, ResponseError};
use crate::services::{decode_role_name, read_buffer_slice};
use async_trait::async_trait;
use std::str;

pub struct EnterGameHandler;

#[async_trait]
impl BillingHandler for EnterGameHandler {
    fn get_type(&self) -> u8 {
        0xA3
    }

    async fn get_response(&self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let offset = 0;
        let request_op_data = request.op_data.as_slice();
        //用户名
        let (username, _) = read_buffer_slice(request_op_data, offset);
        let username_str = str::from_utf8(username).unwrap();
        //角色名
        let (role_nickname, _) = read_buffer_slice(request_op_data, offset);
        println!(
            "user [{}] {} entered game",
            username_str,
            decode_role_name(role_nickname)
        );
        //todo 更新在线状态
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend_from_slice(username);
        response.op_data.push(0x01);
        Ok(response)
    }
}
