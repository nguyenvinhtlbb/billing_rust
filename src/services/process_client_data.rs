use tokio::net::TcpStream;
use crate::common::ResponseError;

pub async fn process_client_data(socket: &mut TcpStream, client_data: &mut Vec<u8>) -> Result<(), ResponseError> {
    Ok(())
}