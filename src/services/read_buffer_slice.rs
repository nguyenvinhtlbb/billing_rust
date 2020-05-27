pub fn read_buffer_slice(buffer: &[u8], offset: usize) -> (&[u8], usize) {
    let mut new_offset = offset;
    let tmp_length = buffer[offset] as usize;
    new_offset += 1;
    let result_value = &buffer[new_offset..new_offset + tmp_length];
    new_offset += tmp_length;
    (result_value, new_offset)
}
