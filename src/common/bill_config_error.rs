use std::error::Error;
use std::fmt::{self, Display, Formatter};

///读取配置文件错误
#[derive(Debug)]
pub enum BillConfigError {
    IoError(tokio::io::Error),
    JsonError(serde_json::Error),
    CustomError(String),
}

impl From<tokio::io::Error> for BillConfigError {
    fn from(err: tokio::io::Error) -> Self {
        BillConfigError::IoError(err)
    }
}

impl From<serde_json::Error> for BillConfigError {
    fn from(err: serde_json::Error) -> Self {
        BillConfigError::JsonError(err)
    }
}

impl From<String> for BillConfigError {
    fn from(err: String) -> Self {
        BillConfigError::CustomError(err)
    }
}

impl Display for BillConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            BillConfigError::IoError(err) => err.fmt(f),
            BillConfigError::JsonError(err) => err.fmt(f),
            BillConfigError::CustomError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for BillConfigError {}
