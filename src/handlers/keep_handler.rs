use crate::common::{
    BillingData, BillingHandler, LoggedUser, LoggedUserCollection, LoggerSender, ResponseError,
};
use crate::log_message;
use crate::services;
use async_trait::async_trait;
use std::str;

pub struct KeepHandler {
    logged_users_collection: LoggedUserCollection,
    logger_sender: LoggerSender,
}

impl KeepHandler {
    pub fn new(logged_users_collection: LoggedUserCollection, logger_sender: LoggerSender) -> Self {
        KeepHandler {
            logged_users_collection,
            logger_sender,
        }
    }
}

#[async_trait]
impl BillingHandler for KeepHandler {
    fn get_type(&self) -> u8 {
        0xA6
    }

    async fn get_response(&mut self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let offset = 0;
        let request_op_data = request.op_data.as_slice();
        //用户名
        let (username, offset) = services::read_buffer_slice(request_op_data, offset);
        let username_str = str::from_utf8(username).unwrap();
        let user_level = {
            let high_value = (request_op_data[offset] as u16) << 8;
            high_value + (request_op_data[offset + 1] as u16)
        };
        //更新用户状态
        let logged_users_guard = self.logged_users_collection.write().await;
        LoggedUser::set_logged_user(logged_users_guard, username_str, true);
        log_message!(
            self.logger_sender,
            Info,
            "keep: user [{}] level {}",
            username_str,
            user_level
        );
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend(username);
        response.op_data.push(0x01);
        Ok(response)
    }
}
