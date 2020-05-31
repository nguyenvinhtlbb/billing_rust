use crate::common::{BillConfig, LoggerSender};
use crate::{log_message, services};
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

mod accept_connection;
mod make_handlers;
mod on_client_connected;
mod show_soft_info;
mod wait_for_shutdown;

/// 运行服务器
pub async fn run_server(server_config: BillConfig, mut logger_sender: LoggerSender) {
    //输出软件信息
    show_soft_info::show_soft_info(&mut logger_sender).await;
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
    let (close_sender, close_receiver) = mpsc::channel::<u8>(1);
    //用于通知所有的连接进行关闭的channel
    let (notify_sender, _) = broadcast::channel::<u8>(1);
    select! {
        _ = accept_connection::accept_connection(&mut listener,&db_pool,&server_config,close_sender,logger_sender.clone(),&notify_sender) => {
            log_message!(logger_sender,Info,"listener stopped");
        }
        value = wait_for_shutdown::wait_for_shutdown(close_receiver) => {
            //通知所有的连接进行关闭,如果不进行通知,那么已建立的TCP连接可能会卡在socket.read(...),导致程序无法退出
            if notify_sender.send(0).is_err() {
                log_message!(logger_sender, Error, "notify close error");
            }
            let quit_way= match value{
                1 => "billing server stopped(by signal)",
                2 => "billing server stopped(by stop command)",
                _ => "unknown way",
            };
            log_message!(logger_sender,Info,"{}",quit_way);
        }
    }
    //释放连接池
    if let Err(err) = db_pool.disconnect().await {
        log_message!(logger_sender, Info, "free database error: {}", err);
    }
}
