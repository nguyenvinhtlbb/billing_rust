use crate::common::BillConfig;
use sqlx::MySqlPool;
use std::time::Duration;

/// 创建数据库连接池
pub async fn create_db_pool(server_config: &BillConfig) -> Result<MySqlPool, sqlx::Error> {
    let conn_string = format!(
        "mysql://{user}:{password}@{host}:{port}/{database}",
        user = server_config.db_user(),
        password = server_config.db_password(),
        host = server_config.db_host(),
        port = server_config.db_port(),
        database = server_config.db_name()
    );
    MySqlPool::builder()
        .connect_timeout(Duration::from_secs(10))
        .min_size(10)
        //.max_size(100)
        .idle_timeout(Duration::from_secs(5 * 60))
        .max_lifetime(Duration::from_secs(30 * 60))
        .build(&conn_string)
        .await
}
