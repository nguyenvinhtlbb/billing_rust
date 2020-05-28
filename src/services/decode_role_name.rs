use encoding_rs::GBK;
use std::borrow::Cow;

/// 昵称解码
pub fn decode_role_name(role_name: &[u8]) -> Cow<str> {
    let (role_name_str, _, _) = GBK.decode(role_name);
    role_name_str
}
