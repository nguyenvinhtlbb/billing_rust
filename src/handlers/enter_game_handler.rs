use crate::common::{
    AuthUser, AuthUsersCollection, BillingData, BillingHandler, Logger, ResponseError,
};
use crate::log_message;
use crate::services::{decode_role_name, read_buffer_slice};
use async_trait::async_trait;
use std::str;
use std::sync::Arc;

pub struct EnterGameHandler {
    auth_users_collection: AuthUsersCollection,
    logger: Arc<Logger>,
}

impl EnterGameHandler {
    pub fn new(auth_users_collection: AuthUsersCollection, logger: Arc<Logger>) -> Self {
        EnterGameHandler {
            auth_users_collection,
            logger,
        }
    }
}

#[async_trait]
impl BillingHandler for EnterGameHandler {
    fn get_type(&self) -> u8 {
        0xA3
    }

    async fn get_response(&self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let offset = 0;
        let request_op_data = request.op_data.as_slice();
        //用户名
        let (username, offset) = read_buffer_slice(request_op_data, offset);
        let username_str = str::from_utf8(username).unwrap();
        //角色名
        let (role_nickname, _) = read_buffer_slice(request_op_data, offset);
        let role_name_str = decode_role_name(role_nickname);
        log_message!(
            self.logger,
            Info,
            "user [{}] {} entered game",
            username_str,
            &role_name_str
        );
        //更新用户状态
        let auth_users_guard = self.auth_users_collection.write().await;
        AuthUser::set_auth_user(auth_users_guard, username_str, true);
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend_from_slice(username);
        response.op_data.push(0x01);
        Ok(response)
    }
}
