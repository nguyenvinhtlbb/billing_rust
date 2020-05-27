use crate::common::BillConfig;
use mysql_async::{OptsBuilder, Pool};

pub fn create_db_pool(server_config: &BillConfig) -> Pool {
    let mut builder = OptsBuilder::new();
    builder
        .ip_or_hostname(server_config.db_host())
        .tcp_port(server_config.db_port())
        .db_name(Some(server_config.db_name()))
        .user(Some(server_config.db_user()))
        .pass(Some(server_config.db_password()));
    Pool::new(builder)
}
