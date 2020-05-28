use crate::common::{BillingData, BillingHandler, ParsePackError, ResponseError};
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

/// 当读取到TCP数据后的处理
pub async fn process_client_data<S: std::hash::BuildHasher>(
    socket: &mut TcpStream,
    client_data: &mut Vec<u8>,
    handlers: &HashMap<u8, Box<dyn BillingHandler>, S>,
) -> Result<(), ResponseError> {
    loop {
        let (billing_data, full_pack_size) =
            match BillingData::read_from_client(client_data.as_slice()) {
                Ok(value) => value,
                Err(err) => match err {
                    ParsePackError::BillingDataNotFull => break,
                    ParsePackError::BillingDataError => return Err(ResponseError::PackError),
                },
            };
        let new_slice = client_data.as_slice();
        *client_data = Vec::from(&new_slice[full_pack_size..]);
        //dbg!(&client_data);
        if billing_data.op_type != 0xA1 {
            dbg!(&billing_data);
        }
        if let Some(bill_handler) = handlers.get(&billing_data.op_type) {
            let response = bill_handler.get_response(&billing_data).await?;
            //dbg!(&response);
            let response_bytes = response.pack_data();
            //dbg!(&response_bytes);
            socket.write_all(&response_bytes).await?;
        } else {
            eprintln!(
                "unknown billing data (op_type={:#04X}) :{:?}",
                billing_data.op_type, &billing_data
            )
        }
    }
    Ok(())
}
