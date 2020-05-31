use crate::common::{BillConfig, LoggerSender};
use crate::log_message;
use mysql_async::Pool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::stream::StreamExt;
use tokio::sync::broadcast::Sender as NotifySender;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

/// 接受TCP连接
pub(super) async fn accept_connection(
    listener: &mut TcpListener,
    db_pool: &Pool,
    server_config: &BillConfig,
    close_sender: Sender<u8>,
    mut logger_sender: LoggerSender,
    notify_receiver: &NotifySender<u8>,
) {
    let stopped_flag = Arc::new(RwLock::new(false));
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        match stream {
            Ok(socket) => {
                let addr = socket.peer_addr().unwrap();
                let allow_ips = server_config.allow_ips();
                //只允许指定的ip连接
                if !allow_ips.is_empty() {
                    let addr_string = addr.ip().to_string();
                    if !allow_ips.contains(&addr_string) {
                        log_message!(logger_sender, Error, "ip {} is not allowed", &addr_string);
                        continue;
                    }
                }
                let handlers = super::make_handlers::make_handlers(
                    server_config,
                    &close_sender,
                    &db_pool,
                    &stopped_flag,
                    &logger_sender,
                );
                let debug_type = server_config.debug_type();
                //在后台处理新的连接
                tokio::spawn(super::on_client_connected::on_client_connected(
                    socket,
                    notify_receiver.subscribe(),
                    logger_sender.clone(),
                    handlers,
                    stopped_flag.clone(),
                    addr,
                    debug_type,
                ));
            }
            Err(err) => {
                log_message!(logger_sender, Error, "accept client error: {}", err);
            }
        }
    }
}
