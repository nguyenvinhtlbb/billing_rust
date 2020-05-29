use super::ParsePackError;
use std::fmt::{self, Debug, Formatter};

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
    /// 读取字节slice,返回读到的数据包和数据包总长度
    pub fn read_from_client(client_data: &[u8]) -> Result<(BillingData, usize), ParsePackError> {
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
        //dbg!(&client_data[2..=3]);
        let op_data_length = ((client_data[2] as usize) << 8) + (client_data[3] as usize) - 3;
        // 数据包的总大小
        let pack_length = min_pack_size + op_data_length;
        if binary_data_length < pack_length {
            // 判断数据包总字节数是否达到
            return Err(ParsePackError::BillingDataNotFull);
        }
        //检测标识尾部
        if !(client_data[pack_length - 2] == mask_data[1]
            && client_data[pack_length - 1] == mask_data[0])
        {
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
        Ok((pack_data, op_data_length + min_pack_size))
    }

    /// 将数据包打包为字节Vec
    pub fn pack_data(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.op_data.len() + 9);
        let mask_data: [u8; 2] = [0xAA, 0x55];
        //头部字节
        result.extend_from_slice(&mask_data);
        //长度高位
        let length_p = (3 + self.op_data.len()) as u16;
        let tmp_data = (length_p >> 8) as u8;
        result.push(tmp_data);
        //长度低位
        let tmp_data = (length_p & 0xff) as u8;
        result.push(tmp_data);
        // append data
        result.push(self.op_type);
        result.extend_from_slice(&self.msg_id);
        if length_p > 3 {
            result.extend_from_slice(self.op_data.as_slice());
        }
        result.push(mask_data[1]);
        result.push(mask_data[0]);
        result
    }
}

impl From<&BillingData> for BillingData {
    fn from(request: &BillingData) -> Self {
        let mut response = Self::default();
        response.op_type = request.op_type;
        response.msg_id = request.msg_id;
        response
    }
}

impl Debug for BillingData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op_type_text = match self.op_type {
            0 => "Close",
            0xA0 => "Connect",
            0xA1 => "Ping",
            0xA2 => "Login",
            0xA3 => "EnterGame",
            0xA4 => "Logout",
            0xA6 => "Keep",
            0xA9 => "Kick",
            0xC5 => "CostLog",
            0xE1 => "ConvertPoint",
            0xE2 => "QueryPoint",
            0xF1 => "Register",
            _ => "Unknown",
        };
        let op_data: Vec<String> = self
            .op_data
            .iter()
            .map(|value| format!("{:02X}", value))
            .collect();
        let op_data_length = op_data.len();
        let op_data = op_data.join(" ");
        let msg_id = format!("{:02X} {:02X}", self.msg_id[0], self.msg_id[1]);
        let raw_pack: Vec<String> = self
            .pack_data()
            .iter()
            .map(|value| format!("{:02X}", value))
            .collect();
        let raw_pack = raw_pack.join(" ");
        write!(
            f,
            "BillingData{{\n\
        \top_type: {:#04X}({}),\n\
        \tmsg_id: [{}],\n\
        \top_data: ({}bytes)[{}]\n\
        }}\n\
        #raw: [{}]",
            self.op_type, op_type_text, msg_id, op_data_length, op_data, raw_pack
        )
    }
}
