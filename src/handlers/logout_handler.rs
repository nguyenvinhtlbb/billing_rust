use crate::common::{
    AuthUser, AuthUsersCollection, BillingData, BillingHandler, Logger, ResponseError,
};
use crate::log_message;
use crate::services::read_buffer_slice;
use async_trait::async_trait;
use std::str;
use std::sync::Arc;

pub struct LogoutHandler {
    auth_users_collection: AuthUsersCollection,
    logger: Arc<Logger>,
}

impl LogoutHandler {
    pub fn new(auth_users_collection: AuthUsersCollection, logger: Arc<Logger>) -> Self {
        LogoutHandler {
            auth_users_collection,
            logger,
        }
    }
}

#[async_trait]
impl BillingHandler for LogoutHandler {
    fn get_type(&self) -> u8 {
        0xA4
    }

    async fn get_response(&self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let offset = 0;
        let request_op_data = request.op_data.as_slice();
        //用户名
        let (username, _) = read_buffer_slice(request_op_data, offset);
        let username_str = str::from_utf8(username).unwrap();
        //更新在线状态
        let auth_users_guard = self.auth_users_collection.write().await;
        AuthUser::remove_user(auth_users_guard, username_str);
        log_message!(self.logger, Info, "user {} logout game", username_str);
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend_from_slice(username);
        response.op_data.push(0x01);
        Ok(response)
    }
}
