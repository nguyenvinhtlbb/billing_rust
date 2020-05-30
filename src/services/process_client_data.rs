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
    //循环读取
    loop {
        let (billing_data, full_pack_size) =
            match BillingData::read_from_client(client_data.as_slice()) {
                //成功读取到一个BillingData,将进行后续处理
                Ok(value) => value,
                Err(err) => match err {
                    // 数据长度不足,跳出loop循环
                    ParsePackError::BillingDataNotFull => break,
                    //数据结构错误
                    ParsePackError::BillingDataError => return Err(ResponseError::PackError),
                },
            };
        //将读取到的字节数据移出
        let new_slice = client_data.as_slice();
        *client_data = Vec::from(&new_slice[full_pack_size..]);
        //用于调试,打印除了0xA1类型的BillingData请求
        if billing_data.op_type != 0xA1 {
            dbg!(&billing_data);
        }
        //查找对应类型的handler
        if let Some(bill_handler) = handlers.get(&billing_data.op_type) {
            // 使用handler从request得到response
            let response = bill_handler.get_response(&billing_data).await?;
            //dbg!(&response);
            // 打包为字节序列
            let response_bytes = response.pack_data();
            //dbg!(&response_bytes);
            // 发送到Client
            socket.write_all(&response_bytes).await?;
        } else {
            // 记录不能处理的类型
            eprintln!(
                "unknown billing data (op_type={:#04X}) :{:?}",
                billing_data.op_type, &billing_data
            )
        }
    }
    Ok(())
}
