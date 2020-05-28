/// 表示处理用户发送的数据发生错误
#[derive(Debug)]
pub enum ResponseError {
    /// 写入错误
    WriteError(tokio::io::Error),
    /// 数据包错误
    PackError,
    /// 数据库出错
    DatabaseError(mysql_async::error::Error),
}

impl From<mysql_async::error::Error> for ResponseError {
    fn from(err: mysql_async::error::Error) -> Self {
        ResponseError::DatabaseError(err)
    }
}

impl From<tokio::io::Error> for ResponseError {
    fn from(err: tokio::io::Error) -> Self {
        ResponseError::WriteError(err)
    }
}
