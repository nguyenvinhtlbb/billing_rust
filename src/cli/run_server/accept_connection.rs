use crate::common::{BillConfig, LoggerSender};
use crate::log_message;
use mysql_async::Pool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

/// 接受TCP连接
pub(super) async fn accept_connection(
    listener: &mut TcpListener,
    db_pool: &Pool,
    server_config: &BillConfig,
    tx: Sender<u8>,
    mut logger_sender: LoggerSender,
) {
    let stopped_flag = Arc::new(RwLock::new(false));
    loop {
        let (socket, addr) = match listener.accept().await {
            Ok(value) => value,
            Err(err) => {
                log_message!(logger_sender, Error, "accept client error: {}", err);
                continue;
            }
        };
        super::on_client_connected::on_client_connected(
            socket,
            addr,
            db_pool,
            &server_config,
            &tx,
            stopped_flag.clone(),
            logger_sender.clone(),
        );
    }
}
