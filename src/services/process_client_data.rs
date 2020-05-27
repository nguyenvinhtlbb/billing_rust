use crate::common::{BillingData, BillingHandler, ParsePackError, ResponseError};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn process_client_data(
    socket: &mut TcpStream,
    client_data: &mut Vec<u8>,
    handlers: &[Box<dyn BillingHandler>],
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
        //dbg!(&billing_data);
        for bill_handler in handlers.iter() {
            if bill_handler.get_type() == billing_data.op_type {
                let response = bill_handler.get_response(&billing_data).await?;
                //dbg!(&response);
                let response_bytes = response.pack_data();
                //dbg!(&response_bytes);
                if let Err(err) = socket.write_all(&response_bytes).await {
                    return Err(ResponseError::WriteError(err));
                }
                break;
            }
        }
    }
    Ok(())
}
