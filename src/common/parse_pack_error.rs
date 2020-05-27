/// 解析数据包的错误
pub enum ParsePackError {
    /// 数据包长度不足
    BillingDataNotFull,
    /// 数据包格式错误
    BillingDataError,
}
