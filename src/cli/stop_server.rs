use crate::common::{BillConfig, BillingData, LoggerSender};
use crate::log_message;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

/// 停止服务器
pub async fn stop_server(server_config: BillConfig, mut logger_sender: LoggerSender) {
    let billing_address = server_config.listen_address();
    let mut stream = match TcpStream::connect(&billing_address).await {
        Ok(value) => value,
        Err(err) => {
            log_message!(logger_sender, Error, "connect error: {}", err);
            return;
        }
    };
    let send_data = BillingData::default().pack_data();
    log_message!(logger_sender, false, Info, "stopping billing server ...");
    if let Err(err) = stream.write_all(&send_data).await {
        log_message!(logger_sender, Error, "stop billing failed: {}", err);
    }
    log_message!(logger_sender, false, Info, "stopped success");
}
