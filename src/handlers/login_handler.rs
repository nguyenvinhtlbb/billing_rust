use crate::common::{BillingData, BillingHandler, ResponseError};
use crate::services::{get_login_result, read_buffer_slice};
use async_trait::async_trait;
use mysql_async::Pool;
use std::str;

pub struct LoginHandler {
    db_pool: Pool,
    auto_reg: bool,
}

impl LoginHandler {
    pub fn new(db_pool: Pool, auto_reg: bool) -> Self {
        LoginHandler { db_pool, auto_reg }
    }
}

#[async_trait]
impl BillingHandler for LoginHandler {
    fn get_type(&self) -> u8 {
        0xA2
    }

    async fn get_response(&self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let offset = 0;
        let request_op_data = request.op_data.as_slice();
        //用户名
        let (username, offset) = read_buffer_slice(request_op_data, offset);
        //密码
        let (password, offset) = read_buffer_slice(request_op_data, offset);
        //登录IP
        let (login_ip, _) = read_buffer_slice(request_op_data, offset);
        //
        let username_str = str::from_utf8(username).unwrap();
        let password_str = str::from_utf8(password).unwrap();
        let mut login_flag = match get_login_result(&self.db_pool, username_str, password_str).await
        {
            Ok(value) => value,
            Err(err) => {
                // 数据库异常
                eprintln!("query error: {}", err);
                6
            }
        };
        // 未启用自动注册
        if !self.auto_reg && login_flag == 9 {
            // 密码错误
            login_flag = 3;
        }
        let login_ip_str = str::from_utf8(login_ip).unwrap();
        println!(
            "user {} try to login from {} : {}",
            username_str, login_ip_str, login_flag
        );
        //
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend_from_slice(username);
        response.op_data.push(login_flag);
        Ok(response)
    }
}
