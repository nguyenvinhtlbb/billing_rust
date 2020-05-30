use crate::common::{BillConfig, LoggerSender};
use crate::log_message;
use crate::services;
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::mpsc;

mod accept_connection;
mod shutdown_signal;

use accept_connection::accept_connection;
use shutdown_signal::shutdown_signal;

mod make_handlers;
mod on_client_connected;

/// 运行服务器
pub async fn run_server(server_config: BillConfig, mut logger_sender: LoggerSender) {
    //dbg!(&server_config);
    //创建tcp服务器
    let listen_address = server_config.listen_address();
    let mut listener = match TcpListener::bind(&listen_address).await {
        Ok(value) => value,
        Err(err) => {
            log_message!(logger_sender, Error, "bind error: {}", err);
            return;
        }
    };
    //连接数据库
    log_message!(logger_sender, Info, "Connecting to database...");
    let db_pool = services::create_db_pool(&server_config);
    match services::get_db_version(&db_pool).await {
        Ok(value) => {
            log_message!(logger_sender, Info, "mysql version: {}", value);
        }
        Err(err) => {
            log_message!(logger_sender, Error, "Database Error: {}", err);
            return;
        }
    };
    log_message!(
        logger_sender,
        Info,
        "billing server run at {}",
        &listen_address
    );
    //用于关闭服务的channel
    let (tx, rx) = mpsc::channel::<u8>(1);
    select! {
        _ = accept_connection(&mut listener,&db_pool,&server_config,tx,logger_sender.clone()) => {
            log_message!(logger_sender,Info,"listener stopped");
        }
        value = shutdown_signal(rx) => {
            let quit_way= match value{
                1 => "billing server stopped(by signal)",
                2 => "billing server stopped(by stop command)",
                _ => "unknown way",
            };
            log_message!(logger_sender,Info,"{}",quit_way);
        }
    }
}
