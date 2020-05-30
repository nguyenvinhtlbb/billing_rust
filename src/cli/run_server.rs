use crate::common::BillConfig;
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
pub async fn run_server(server_config: BillConfig) {
    //dbg!(&server_config);
    //创建tcp服务器
    let listen_address = server_config.listen_address();
    let mut listener = match TcpListener::bind(&listen_address).await {
        Ok(value) => value,
        Err(err) => {
            eprintln!("bind error: {}", err);
            return;
        }
    };
    //连接数据库
    println!("Connecting to database...");
    let db_pool = services::create_db_pool(&server_config);
    match services::get_db_version(&db_pool).await {
        Ok(value) => println!("mysql version: {}", value),
        Err(err) => {
            eprintln!("Database Error: {}", err);
            return;
        }
    };
    println!("billing server run at {}", &listen_address);
    //用于关闭服务的channel
    let (tx, rx) = mpsc::channel::<u8>(1);
    select! {
        _ = accept_connection(&mut listener,&db_pool,&server_config,tx) => {
            println!("listener stopped");
        }
        value = shutdown_signal(rx) => {
            match value{
                1 => println!("billing server stopped(by signal)"),
                2 => println!("billing server stopped(by stop command)"),
                _ => println!("unknown way"),
            }
        }
    }
}
