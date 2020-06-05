use crate::common::{
    BillingData, BillingHandler, LoggedUser, LoggedUserCollection, LoggerSender, ResponseError,
};
use crate::log_message;
use crate::models::Account;
use crate::services;
use async_trait::async_trait;
use sqlx::MySqlPool;
use std::cmp::min;
use std::str;

pub struct ConvertPointHandler {
    db_pool: MySqlPool,
    convert_number: i32,
    logged_users_collection: LoggedUserCollection,
    logger_sender: LoggerSender,
}

impl ConvertPointHandler {
    pub fn new(
        db_pool: MySqlPool,
        convert_number: i32,
        logged_users_collection: LoggedUserCollection,
        logger_sender: LoggerSender,
    ) -> Self {
        ConvertPointHandler {
            db_pool,
            convert_number,
            logged_users_collection,
            logger_sender,
        }
    }
}

#[async_trait]
impl BillingHandler for ConvertPointHandler {
    fn get_type(&self) -> u8 {
        0xE1
    }

    async fn get_response(&mut self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let offset = 0;
        let request_op_data = request.op_data.as_slice();
        //用户名
        let (username, offset) = services::read_buffer_slice(request_op_data, offset);
        //登录IP
        let (client_ip, offset) = services::read_buffer_slice(request_op_data, offset);
        //角色名
        let (role_nickname, offset) = services::read_buffer_slice(request_op_data, offset);
        //order id
        let (order_id_bytes, offset) = {
            let item_length = 21;
            (
                &request_op_data[offset..offset + item_length],
                offset + item_length,
            )
        };
        //extra bytes
        let (extra_data_bytes, offset) = {
            let item_length = 6;
            (
                &request_op_data[offset..offset + item_length],
                offset + item_length,
            )
        };
        let mut need_point = {
            let offset = offset + 2;
            let mut result_value = 0;
            for i in 0..4 {
                result_value += if i < 3 {
                    (request_op_data[offset + i] as i32) << ((8 * (3 - i)) as i32)
                } else {
                    request_op_data[offset + i] as i32
                };
            }
            result_value / self.convert_number
        };
        //每次兑换的最大点数
        let max_convert_point = 0xffff;
        if need_point > max_convert_point {
            need_point = max_convert_point;
        }
        // 查询账号点数
        let username_str = str::from_utf8(username).unwrap();
        let user_point_value = match Account::get_by_username(&self.db_pool, username_str).await {
            Ok(account_result) => match account_result {
                Some(account_info) => account_info.point(),
                None => 0,
            },
            Err(err) => {
                log_message!(
                    self.logger_sender,
                    Error,
                    "get account {} info error {}",
                    username_str,
                    err
                );
                0
            }
        };
        // 用户点数不能为负数
        let user_point_value = if user_point_value < 0 {
            0
        } else {
            user_point_value
        };
        let cost_point = min(need_point, user_point_value);
        // 执行兑换
        let cost_point = match Account::convert_point(username_str, &self.db_pool, cost_point).await
        {
            Ok(_) => cost_point,
            Err(err) => {
                log_message!(
                    self.logger_sender,
                    Error,
                    "account {} convert point error {}",
                    username_str,
                    err
                );
                0
            }
        };
        //更新用户在线状态
        let logged_users_guard = self.logged_users_collection.write().await;
        LoggedUser::set_logged_user(logged_users_guard, username_str, true);
        let client_ip_str = str::from_utf8(client_ip).unwrap();
        log_message!(
            self.logger_sender,
            Info,
            "user [{}] {}(ip: {}) point total [{}], need point [{}]: {}-{}={}",
            username_str,
            services::decode_role_name(role_nickname),
            client_ip_str,
            user_point_value,
            need_point,
            user_point_value,
            cost_point,
            user_point_value - cost_point
        );
        //
        let mut response: BillingData = request.into();
        response.op_data.push(username.len() as u8);
        response.op_data.extend(username);
        response.op_data.extend(order_id_bytes);
        response.op_data.extend(&[0x00, 0x00, 0x00, 0x03, 0xE8]);
        response.op_data.extend(extra_data_bytes);
        let tmp_data = ((cost_point & 0xff00) >> 8) as u8;
        response.op_data.push(tmp_data);
        let tmp_data = (cost_point & 0xff) as u8;
        response.op_data.push(tmp_data);
        Ok(response)
    }
}
