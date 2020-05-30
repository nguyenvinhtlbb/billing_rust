use crate::common::{BillingData, BillingHandler, Logger, ResponseError};
use crate::log_message;
use crate::services::{get_register_result, read_buffer_slice};
use async_trait::async_trait;
use mysql_async::Pool;
use std::str;
use std::sync::Arc;

pub struct RegisterHandler {
    db_pool: Pool,
    logger: Arc<Logger>,
}

impl RegisterHandler {
    pub fn new(db_pool: Pool, logger: Arc<Logger>) -> Self {
        RegisterHandler { db_pool, logger }
    }
}

#[async_trait]
impl BillingHandler for RegisterHandler {
    fn get_type(&self) -> u8 {
        0xF1
    }

    async fn get_response(&self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let offset = 0;
        let request_op_data = request.op_data.as_slice();
        //用户名
        let (username, offset) = read_buffer_slice(request_op_data, offset);
        //超级密码
        let (super_password, offset) = read_buffer_slice(request_op_data, offset);
        //密码
        let (password, offset) = read_buffer_slice(request_op_data, offset);
        //登录IP
        let (client_ip, offset) = read_buffer_slice(request_op_data, offset);
        //email
        let (email, _) = read_buffer_slice(request_op_data, offset);
        //
        let username_str = str::from_utf8(username).unwrap();
        let email_str = str::from_utf8(email).unwrap();
        let password_str = str::from_utf8(password).unwrap();
        let super_password_str = str::from_utf8(super_password).unwrap();
        let register_flag = match get_register_result(
            &self.db_pool,
            username_str,
            password_str,
            super_password_str,
            email_str,
        )
        .await
        {
            Ok(value) => value,
            Err(err) => {
                // 数据库异常
                log_message!(self.logger, Error, "query error: {}", err);
                4
            }
        };
        let client_ip_str = str::from_utf8(client_ip).unwrap();
        let register_flag_str = if register_flag == 1 {
            "success"
        } else {
            "error"
        };
        log_message!(
            self.logger,
            Info,
            "user {}({}) try to register from {} : {}",
            username_str,
            email_str,
            client_ip_str,
            register_flag_str
        );
        //
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend_from_slice(username);
        response.op_data.push(register_flag);
        Ok(response)
    }
}
