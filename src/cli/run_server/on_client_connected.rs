use crate::common::{BillDebugType, BillingHandler, LoggerSender, ResponseError};
use crate::log_message;
use crate::services;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

pub(super) async fn on_client_connected(
    mut socket: TcpStream,
    client_address: SocketAddr,
    mut logger_sender: LoggerSender,
    mut handlers: HashMap<u8, Box<dyn BillingHandler>>,
    stopped_flag: Arc<RwLock<bool>>,
    debug_type: BillDebugType,
) {
    log_message!(logger_sender, Info, "client {} connected", &client_address);
    let mut buf = [0; 1024];
    let mut client_data: Vec<u8> = vec![];
    // In a loop, read data from the client
    loop {
        let read_result = socket.read(&mut buf).await;
        let n = match read_result {
            // socket closed
            Err(_) | Ok(0) => {
                //如果服务是否已经停止,则直接返回
                {
                    let stopped_flag_guard = stopped_flag.read().await;
                    if *stopped_flag_guard {
                        return;
                    }
                }
                //否则,记录错误信息
                if let Err(err) = read_result {
                    log_message!(
                        logger_sender,
                        Error,
                        "failed to read from socket; err = {:?}",
                        err
                    );
                } else {
                    log_message!(
                        logger_sender,
                        Error,
                        "client {} disconnected",
                        &client_address
                    );
                }
                return;
            }
            Ok(n) => n,
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
}
