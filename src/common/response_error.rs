/// 表示处理用户发送的数据发生错误
#[derive(Debug)]
pub enum ResponseError {
    /// 写入错误
    WriteError(tokio::io::Error),
    /// 数据包错误
    PackError,
    /// 数据库出错
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for ResponseError {
    fn from(err: sqlx::Error) -> Self {
        ResponseError::DatabaseError(err)
    }
}

impl From<tokio::io::Error> for ResponseError {
    fn from(err: tokio::io::Error) -> Self {
        ResponseError::WriteError(err)
    }
}
