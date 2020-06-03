use crate::common::{
    BillingData, BillingHandler, LoggedUser, LoggedUserCollection, LoggerSender, ResponseError,
};
use crate::log_message;
use crate::services;
use async_trait::async_trait;
use std::str;

pub struct EnterGameHandler {
    logged_users_collection: LoggedUserCollection,
    logger_sender: LoggerSender,
}

impl EnterGameHandler {
    pub fn new(logged_users_collection: LoggedUserCollection, logger_sender: LoggerSender) -> Self {
        EnterGameHandler {
            logged_users_collection,
            logger_sender,
        }
    }
}

#[async_trait]
impl BillingHandler for EnterGameHandler {
    fn get_type(&self) -> u8 {
        0xA3
    }

    async fn get_response(&mut self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let offset = 0;
        let request_op_data = request.op_data.as_slice();
        //用户名
        let (username, offset) = services::read_buffer_slice(request_op_data, offset);
        let username_str = str::from_utf8(username).unwrap();
        //角色名
        let (role_nickname, _) = services::read_buffer_slice(request_op_data, offset);
        let role_name_str = services::decode_role_name(role_nickname);
        log_message!(
            self.logger_sender,
            Info,
            "user [{}] {} entered game",
            username_str,
            &role_name_str
        );
        //更新用户状态
        let logged_users_guard = self.logged_users_collection.write().await;
        LoggedUser::set_logged_user(logged_users_guard, username_str, true);
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend_from_slice(username);
        response.op_data.push(0x01);
        Ok(response)
    }
}
