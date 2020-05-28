use crate::common::{BillConfig, BillingHandler, ResponseError};
use crate::handlers::{
    CloseHandler, ConnectHandler, ConvertPointHandler, CostLogHandler, EnterGameHandler,
    KeepHandler, KickHandler, LoginHandler, LogoutHandler, PingHandler, QueryPointHandler,
    RegisterHandler,
};
use crate::services;
use mysql_async::Pool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;

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
        _ = process_listener(&mut listener,&db_pool,&server_config,tx) => {
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

/// 添加handler的宏
macro_rules! add_handler {
    ($handler_map:ident,$($handler:expr ),*) => {
        $(
            let tmp_handler = Box::new($handler);
            //println!("op_type={:#04X}", $handler.get_type());
            $handler_map.insert($handler.get_type(), tmp_handler);
        )*
    };
}

async fn shutdown_signal(mut rx: Receiver<u8>) -> i32 {
    select! {
        // Wait for the CTRL+C signal
         _ = tokio::signal::ctrl_c() => {
            1
         }
         // Wait for stop command
        _ = rx.recv() => {
            2
        }
    }
}

async fn process_listener(
    listener: &mut TcpListener,
    db_pool: &Pool,
    server_config: &BillConfig,
    tx: Sender<u8>,
) {
    let stopped_flag = Arc::new(Mutex::new(false));
    loop {
        let (socket, addr) = match listener.accept().await {
            Ok(value) => value,
            Err(err) => {
                eprintln!("accept client error: {}", err);
                continue;
            }
        };
        process_client_socket(
            socket,
            addr,
            db_pool.clone(),
            &server_config,
            tx.clone(),
            stopped_flag.clone(),
        );
    }
}

fn process_client_socket(
    mut socket: TcpStream,
    client_address: SocketAddr,
    db_pool: Pool,
    server_config: &BillConfig,
    tx: Sender<u8>,
    stopped_flag: Arc<Mutex<bool>>,
) {
    let auto_reg = server_config.auto_reg();
    let convert_number = server_config.transfer_number();
    tokio::spawn(async move {
        println!("client {} connected", &client_address);
        let mut buf = [0; 1024];
        let mut client_data: Vec<u8> = vec![];
        let mut handlers: HashMap<u8, Box<dyn BillingHandler>> = HashMap::new();
        //向handlers Map中添加handler
        add_handler!(
            handlers,
            CloseHandler::new(tx.clone(), stopped_flag.clone()),
            ConnectHandler,
            PingHandler,
            LoginHandler::new(db_pool.clone(), auto_reg),
            EnterGameHandler,
            LogoutHandler,
            KeepHandler,
            KickHandler,
            CostLogHandler,
            ConvertPointHandler::new(db_pool.clone(), convert_number),
            QueryPointHandler::new(db_pool.clone()),
            RegisterHandler::new(db_pool.clone())
        );
        // In a loop, read data from the socket and write the data back.
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
                    let stopped_flag_guard = stopped_flag.lock().await;
                    if !*stopped_flag_guard {
                        eprintln!("failed to read from socket; err = {:?}", e);
                    }
                    return;
                }
            };
            client_data.extend_from_slice(&buf[..n]);
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
                //eprintln!("failed to write to socket; err = {:?}", err);
                return;
            }
            // Write the data back
            /*if let Err(e) = socket.write_all(&buf[0..n]).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                return;
            }*/
        }
    });
}
