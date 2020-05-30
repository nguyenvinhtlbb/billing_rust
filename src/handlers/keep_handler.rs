use crate::common::{
    AuthUser, AuthUsersCollection, BillingData, BillingHandler, LoggerSender, ResponseError,
};
use crate::log_message;
use crate::services::read_buffer_slice;
use async_trait::async_trait;
use std::str;

use tokio::sync::Mutex;

pub struct KeepHandler {
    auth_users_collection: AuthUsersCollection,
    logger_sender: Mutex<LoggerSender>,
}

impl KeepHandler {
    pub fn new(auth_users_collection: AuthUsersCollection, logger_sender: LoggerSender) -> Self {
        KeepHandler {
            auth_users_collection,
            logger_sender: Mutex::new(logger_sender),
        }
    }
}

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
            let high_value = (request_op_data[offset] as u16) << 8;
            high_value + (request_op_data[offset + 1] as u16)
        };
        //更新用户状态
        let auth_users_guard = self.auth_users_collection.write().await;
        AuthUser::set_auth_user(auth_users_guard, username_str, true);
        let mut logger_sender = self.logger_sender.lock().await;
        log_message!(
            logger_sender,
            Info,
            "keep: user [{}] level {}",
            username_str,
            user_level
        );
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend_from_slice(username);
        response.op_data.push(0x01);
        Ok(response)
    }
}
