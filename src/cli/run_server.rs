use crate::common::BillConfig;
use crate::services;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;

/// 运行服务器
pub async fn run_server(server_config: BillConfig) {
    dbg!(&server_config);
    //连接数据库
    let db_pool = services::create_db_pool(&server_config);
    match services::get_db_version(&db_pool).await {
        Ok(value) => {
            println!("mysql version: {}", value)
        }
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
    loop {
        let (socket, addr) = match listener.accept().await {
            Ok(value) => value,
            Err(err) => {
                eprintln!("accept client error: {}", err);
                continue;
            }
        };
        process_client_socket(socket, addr);
    }
}

fn process_client_socket(mut socket: TcpStream, client_address: SocketAddr) {
    tokio::spawn(async move {
        println!("client {} connected", &client_address);
        let mut buf = [0; 1024];
        let mut client_data: Vec<u8> = vec![];

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
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            client_data.extend_from_slice(&buf[..n]);
            dbg!(&client_data);
            if let Err(err) = services::process_client_data(&mut socket, &mut client_data).await {
                eprintln!("failed to write to socket; err = {:?}", err);
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