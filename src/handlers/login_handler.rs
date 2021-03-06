use crate::common::{
    BillingData, BillingHandler, LoggedUser, LoggedUserCollection, LoggerSender, ResponseError,
};
use crate::log_message;
use crate::services;
use async_trait::async_trait;
use sqlx::MySqlPool;
use std::str;

pub struct LoginHandler {
    db_pool: MySqlPool,
    auto_reg: bool,
    logged_users_collection: LoggedUserCollection,
    logger_sender: LoggerSender,
}

impl LoginHandler {
    pub fn new(
        db_pool: MySqlPool,
        auto_reg: bool,
        logged_users_collection: LoggedUserCollection,
        logger_sender: LoggerSender,
    ) -> Self {
        LoginHandler {
            db_pool,
            auto_reg,
            logged_users_collection,
            logger_sender,
        }
    }
}

#[async_trait]
impl BillingHandler for LoginHandler {
    fn get_type(&self) -> u8 {
        0xA2
    }

    async fn get_response(&mut self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let offset = 0;
        let request_op_data = request.op_data.as_slice();
        //用户名
        let (username, offset) = services::read_buffer_slice(request_op_data, offset);
        //密码
        let (password, offset) = services::read_buffer_slice(request_op_data, offset);
        //登录IP
        let (login_ip, offset) = services::read_buffer_slice(request_op_data, offset);
        //用户级别:2字节(skip)
        //密保key+value:12字节(skip)
        //用户电脑的MAC地址MD5 32个字节
        let offset = offset + 14;
        let mac_hash = &request_op_data[offset..offset + 32];
        let mac_hash_str = str::from_utf8(mac_hash).unwrap();
        //
        let username_str = str::from_utf8(username).unwrap();
        let password_str = str::from_utf8(password).unwrap();
        let mut login_flag =
            match services::get_login_result(&self.db_pool, username_str, password_str).await {
                Ok(value) => value,
                Err(err) => {
                    // 数据库异常
                    log_message!(self.logger_sender, Error, "query error: {}", err);
                    6
                }
            };
        let login_ip_str = str::from_utf8(login_ip).unwrap();
        // 登录成功
        if login_flag == 1 {
            let logged_users_guard = self.logged_users_collection.read().await;
            // 有角色在线
            if LoggedUser::is_role_online(logged_users_guard, username_str) {
                login_flag = 4;
            } else {
                //更新用户状态
                let logged_users_guard = self.logged_users_collection.write().await;
                LoggedUser::set_logged_user(logged_users_guard, username_str, false);
            }
        }
        // 未启用自动注册
        else if login_flag == 9 && !self.auto_reg {
            // 密码错误
            login_flag = 3;
        }
        let login_flag_str = match login_flag {
            1 => "success",
            3 => "password error",
            4 => "role online",
            6 => "system error",
            7 => "account locked",
            9 => "user does not exists(go to register)",
            _ => "unknown",
        };
        log_message!(
            self.logger_sender,
            Info,
            "user {} try to login from {} MD5(MAC) = {} : {}",
            username_str,
            login_ip_str,
            mac_hash_str,
            login_flag_str
        );
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend(username);
        response.op_data.push(login_flag);
        Ok(response)
    }
}
