use sqlx::mysql::MySqlQueryAs;
use sqlx::MySqlPool;

/// 获取数据库版本信息
pub async fn get_db_version(db_pool: &MySqlPool) -> Result<String, sqlx::Error> {
    let version_info: (String,) = sqlx::query_as("SELECT VERSION() AS ver")
        .fetch_one(db_pool)
        .await?;
    Ok(version_info.0)
}
