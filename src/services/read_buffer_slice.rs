/// 读取buffer,返回读到的字节slice和新的offset
///
/// 用于[u8][.........] => 一字节长度+指定长度数据的buffer结构
pub fn read_buffer_slice(buffer: &[u8], offset: usize) -> (&[u8], usize) {
    let mut new_offset = offset;
    //读取长度: 1字节
    let tmp_length = buffer[offset] as usize;
    new_offset += 1;
    //读取slice:长度=tmp_length
    let result_value = &buffer[new_offset..new_offset + tmp_length];
    new_offset += tmp_length;
    (result_value, new_offset)
}
