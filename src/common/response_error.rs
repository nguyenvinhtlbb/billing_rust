/// 表示处理用户发送的数据发生错误
#[derive(Debug)]
pub enum ResponseError {
    /// 写入错误
    WriteError(tokio::io::Error),
    /// 数据包错误
    PackError,
}