use mysql_async::prelude::Queryable;
use mysql_async::{Pool, QueryResult};

/// 获取数据库版本信息
pub async fn get_db_version(db_pool: &Pool) -> Result<String, mysql_async::error::Error> {
    let conn = db_pool.get_conn().await?;
    let query_result: QueryResult<_, _> = conn.query("SELECT VERSION() AS ver").await?;
    let (_, rows) = query_result.map(|row| row.get("ver").unwrap()).await?;
    let version: &String = rows.get(0).unwrap();
    Ok(version.to_string())
}
