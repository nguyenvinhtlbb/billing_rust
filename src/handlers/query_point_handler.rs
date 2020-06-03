use crate::common::{
    BillingData, BillingHandler, LoggedUser, LoggedUserCollection, LoggerSender, ResponseError,
};
use crate::log_message;
use crate::models::Account;
use crate::services;
use async_trait::async_trait;
use mysql_async::Pool;
use std::str;

pub struct QueryPointHandler {
    db_pool: Pool,
    logged_users_collection: LoggedUserCollection,
    logger_sender: LoggerSender,
}

impl QueryPointHandler {
    pub fn new(
        db_pool: Pool,
        logged_users_collection: LoggedUserCollection,
        logger_sender: LoggerSender,
    ) -> Self {
        QueryPointHandler {
            db_pool,
            logged_users_collection,
            logger_sender,
        }
    }
}

#[async_trait]
impl BillingHandler for QueryPointHandler {
    fn get_type(&self) -> u8 {
        0xE2
    }

    async fn get_response(&mut self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let offset = 0;
        let request_op_data = request.op_data.as_slice();
        //用户名
        let (username, offset) = services::read_buffer_slice(request_op_data, offset);
        //登录IP
        let (client_ip, offset) = services::read_buffer_slice(request_op_data, offset);
        //角色名
        let (role_nickname, _offset) = services::read_buffer_slice(request_op_data, offset);
        //
        let username_str = str::from_utf8(username).unwrap();
        let client_ip_str = str::from_utf8(client_ip).unwrap();
        let account_result = Account::get_by_username(&self.db_pool, username_str).await?;
        let point_value = match account_result {
            Some(account_info) => account_info.point(),
            None => 0,
        };
        //更新用户在线状态
        let logged_users_guard = self.logged_users_collection.write().await;
        LoggedUser::set_logged_user(logged_users_guard, username_str, true);
        let role_name_str = services::decode_role_name(role_nickname);
        log_message!(
            self.logger_sender,
            Info,
            "user [{}] {} query point ({}) at {}",
            username_str,
            &role_name_str,
            point_value,
            client_ip_str
        );
        //
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend(username);
        //返回值的处理
        let point_value = (point_value + 1) * 1000;
        //将point值拆分为4个u8
        for i in 0..4 {
            let tmp_value = if i < 3 {
                point_value >> ((3 - i) * 8)
            } else {
                point_value
            };
            let tmp_byte = (tmp_value & 0xff) as u8;
            response.op_data.push(tmp_byte);
        }
        Ok(response)
    }
}
