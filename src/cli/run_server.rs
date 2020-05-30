use crate::common::{AuthUsersCollection, BillConfig, BillingHandler, ResponseError};

use crate::services;
use mysql_async::Pool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::RwLock;
mod shutdown_signal;
mod accept_connection;
use shutdown_signal::shutdown_signal;
use accept_connection::accept_connection;
mod on_client_connected;
mod make_handlers;

/// 运行服务器
pub async fn run_server(server_config: BillConfig) {
    dbg!(&server_config);
    //连接数据库
    let db_pool = services::create_db_pool(&server_config);
    match services::get_db_version(&db_pool).await {
        Ok(value) => println!("mysql version: {}", value),
        Err(err) => {
            eprintln!("Database Error: {}", err);
            return;
        }
    };
    //创建tcp服务器
    let listen_address = server_config.listen_address();
    let mut listener = match TcpListener::bind(&listen_address).await {
        Ok(value) => value,
        Err(err) => {
            eprintln!("bind error: {}", err);
            return;
        }
    };
    println!("server run at {}", &listen_address);
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