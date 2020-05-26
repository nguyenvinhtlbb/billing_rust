use super::ParsePackError;

/// 数据包
#[derive(Default)]
pub struct BillingData {
    /// 类型
    pub op_type: u8,
    /// 消息id
    pub msg_id: [u8; 2],
    /// 负载数据
    pub op_data: Vec<u8>,
}

impl BillingData {
    pub fn read_from_client(client_data: &[u8]) -> Result<BillingData, ParsePackError> {
        if client_data.is_empty() {
            return Err(ParsePackError::BillingDataNotFull);
        }
        // 数据包最小长度
        let min_pack_size = 9;
        let binary_data_length = client_data.len();
        if binary_data_length < min_pack_size {
            return Err(ParsePackError::BillingDataNotFull);
        }
        // 检测标识头部
        let mask_data: [u8; 2] = [0xAA, 0x55];
        if client_data[0] != mask_data[0] || client_data[1] != mask_data[1] {
            // 头部数据错误
            return Err(ParsePackError::BillingDataError);
        }
        //负载数据长度(u2)
        // 负载数据长度需要减去一字节类型标识、两字节的id
        let op_data_length = (client_data[2] as usize) << 8 + (client_data[3] as usize) - 3;
        // 数据包的总大小
        let pack_length = min_pack_size + op_data_length;
        if binary_data_length < pack_length {
            // 判断数据包总字节数是否达到
            return Err(ParsePackError::BillingDataNotFull);
        }
        //检测标识尾部
        if !(client_data[pack_length - 2] == mask_data[1] && client_data[pack_length - 1] == mask_data[0]) {
            // 尾部数据错误
            return Err(ParsePackError::BillingDataError);
        }
        let mut pack_data = BillingData::default();
        // 类型标识(u1)
        pack_data.op_type = client_data[4];
        // 消息id(u2)
        pack_data.msg_id[0] = client_data[5];
        pack_data.msg_id[1] = client_data[6];
        // 负载数据
        if op_data_length > 0 {
            pack_data.op_data = Vec::from(&client_data[7..7 + op_data_length]);
        }
        ////
        Ok(pack_data)
    }

    pub fn len(&self) -> usize {
        self.op_data.len() + 9
    }
}