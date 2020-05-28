use crate::common::{BillConfig, BillingData};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

/// 停止服务器
pub async fn stop_server(server_config: BillConfig) {
    let billing_address = server_config.listen_address();
    let mut stream = match TcpStream::connect(&billing_address).await {
        Ok(value) => value,
        Err(err) => {
            eprintln!("connect error: {}", err);
            return;
        }
    };
    let send_data = BillingData::default().pack_data();
    println!("stopping billing server ...");
    if let Err(err) = stream.write_all(&send_data).await {
        eprintln!("stop billing failed: {}", err);
    }
    println!("stopped success");
}
