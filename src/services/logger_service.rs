use crate::common::{LogMessageType, Logger};
use std::path::PathBuf;
use tokio::sync::mpsc::Receiver;

/// 异步记录日志
pub async fn logger_service(
    exe_dir: PathBuf,
    mut logger_receiver: Receiver<(bool, LogMessageType, String)>,
) {
    let log_file_path = exe_dir.join("log.log");
    let logger = Logger::new(log_file_path);
    while let Some((log_to_file, message_type, message)) = logger_receiver.recv().await {
        logger.log(log_to_file, message_type, &message).await;
    }
}
