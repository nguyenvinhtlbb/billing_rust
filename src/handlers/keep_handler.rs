use crate::common::{
    AuthUser, AuthUsersCollection, BillingData, BillingHandler, Logger, ResponseError,
};
use crate::log_message;
use crate::services::read_buffer_slice;
use async_trait::async_trait;
use std::str;
use std::sync::Arc;

pub struct KeepHandler {
    auth_users_collection: AuthUsersCollection,
    logger: Arc<Logger>,
}

impl KeepHandler {
    pub fn new(auth_users_collection: AuthUsersCollection, logger: Arc<Logger>) -> Self {
        KeepHandler {
            auth_users_collection,
            logger,
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
        log_message!(
            self.logger,
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
