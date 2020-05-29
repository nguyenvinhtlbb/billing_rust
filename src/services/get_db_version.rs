use mysql_async::prelude::Queryable;
use mysql_async::{Pool, Row};

/// 获取数据库版本信息
pub async fn get_db_version(db_pool: &Pool) -> Result<String, mysql_async::error::Error> {
    let conn = db_pool.get_conn().await?;
    let (_, option_version) = conn.first::<_, Row>("SELECT VERSION() AS ver").await?;
    let version: String = match option_version {
        Some(row) => row.get("ver").unwrap(),
        None => "unknown".to_string(),
    };
    Ok(version)
}
