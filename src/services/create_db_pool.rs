use crate::common::BillConfig;
use mysql_async::{OptsBuilder, Pool, PoolConstraints, PoolOptions};
use tokio::time::Duration;

/// 创建数据库连接池
pub fn create_db_pool(server_config: &BillConfig) -> Pool {
    //最小、最大连接数
    let constraints = PoolConstraints::new(2, 100).unwrap();
    let mut pool_options = PoolOptions::with_constraints(constraints);
    //超出最小连接数时，连接的最长闲置时间
    pool_options.set_inactive_connection_ttl(Duration::from_secs(120));
    let mut builder = OptsBuilder::new();
    builder
        .pool_options(pool_options)
        .prefer_socket(false)
        .ip_or_hostname(server_config.db_host())
        .tcp_port(server_config.db_port())
        .db_name(Some(server_config.db_name()))
        .user(Some(server_config.db_user()))
        .pass(Some(server_config.db_password()))
        //连接池中的连接最长闲置时间
        .conn_ttl(Duration::from_secs(600));
    //dbg!(&builder);
    Pool::new(builder)
}
