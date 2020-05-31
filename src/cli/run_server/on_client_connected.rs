use crate::common::{BillConfig, LoggerSender, ResponseError};
use crate::log_message;
use crate::services;
use mysql_async::Pool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

pub(super) fn on_client_connected(
    mut socket: TcpStream,
    client_address: SocketAddr,
    db_pool: &Pool,
    server_config: &BillConfig,
    close_sender: &Sender<u8>,
    stopped_flag: Arc<RwLock<bool>>,
    mut logger_sender: LoggerSender,
) {
    let mut handlers = super::make_handlers::make_handlers(
        server_config,
        &close_sender,
        &db_pool,
        &stopped_flag,
        &logger_sender,
    );
    let debug_type = server_config.debug_type();
    tokio::spawn(async move {
        log_message!(logger_sender, Info, "client {} connected", &client_address);
        let mut buf = [0; 1024];
        let mut client_data: Vec<u8> = vec![];
        // In a loop, read data from the client
        loop {
            let n = match socket.read(&mut buf).await {
                // socket closed
                Ok(n) => {
                    if n == 0 {
                        log_message!(
                            logger_sender,
                            Error,
                            "client {} disconnected",
                            &client_address
                        );
                        return;
                    }
                    n
                }
                Err(e) => {
                    // 如果是主动停止服务的,则忽略错误
                    let stopped_flag_guard = stopped_flag.read().await;
                    if !*stopped_flag_guard {
                        log_message!(
                            logger_sender,
                            Error,
                            "failed to read from socket; err = {:?}",
                            e
                        );
                    }
                    return;
                }
            };
            //将读取到数据附加到client_data后面
            client_data.extend_from_slice(&buf[..n]);
            //处理读取到的数据,如果出现错误则直接返回(断开连接)
            if let Err(err) = services::process_client_data(
                &mut socket,
                &mut client_data,
                &mut handlers,
                &mut logger_sender,
                debug_type,
            )
            .await
            {
                let message = match err {
                    ResponseError::WriteError(err) => {
                        format!("failed to write to socket; err = {}", err)
                    }
                    ResponseError::PackError => "invalid pack data".to_string(),
                    ResponseError::DatabaseError(err) => format!("database error: {}", err),
                };
                log_message!(logger_sender, Error, "{}", message);
                return;
            }
        }
    });
}
