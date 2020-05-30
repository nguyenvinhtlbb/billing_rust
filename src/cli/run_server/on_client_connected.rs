use tokio::net::TcpStream;
use std::net::SocketAddr;
use mysql_async::Pool;
use crate::common::{BillConfig, AuthUsersCollection, BillingHandler, ResponseError};
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use crate::services;

pub(super) fn on_client_connected(
    mut socket: TcpStream,
    client_address: SocketAddr,
    db_pool: &Pool,
    server_config: &BillConfig,
    tx: &Sender<u8>,
    stopped_flag: Arc<RwLock<bool>>,
) {
    let handlers = super::make_handlers::make_handlers(server_config, &tx, &db_pool, &stopped_flag);
    tokio::spawn(async move {
        println!("client {} connected", &client_address);
        let mut buf = [0; 1024];
        let mut client_data: Vec<u8> = vec![];
        // In a loop, read data from the client
        loop {
            let n = match socket.read(&mut buf).await {
                // socket closed
                Ok(n) => {
                    if n == 0 {
                        eprintln!("client {} disconnected", &client_address);
                        return;
                    }
                    n
                }
                Err(e) => {
                    let stopped_flag_guard = stopped_flag.read().await;
                    if !*stopped_flag_guard {
                        eprintln!("failed to read from socket; err = {:?}", e);
                    }
                    return;
                }
            };
            //将读取到数据附加到client_data后面
            client_data.extend_from_slice(&buf[..n]);
            //处理读取到的数据,如果出现错误则直接返回(断开连接)
            if let Err(err) =
            services::process_client_data(&mut socket, &mut client_data, &handlers).await
            {
                let message = match err {
                    ResponseError::WriteError(err) => {
                        format!("failed to write to socket; err = {}", err)
                    }
                    ResponseError::PackError => "invalid pack data".to_string(),
                    ResponseError::DatabaseError(err) => format!("database error: {}", err),
                };
                eprintln!("{}", message);
                return;
            }
        }
    });
}