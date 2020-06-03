use encoding_rs::GBK;
use std::borrow::Cow;

/// 昵称解码
pub fn decode_role_name(role_name: &[u8]) -> Cow<str> {
    let (role_name_str, _, had_errors) = GBK.decode(role_name);
    if had_errors {
        let tmp_collection: Vec<String> =
            role_name.iter().map(|s| format!("\\{:02X}", s)).collect();
        Cow::from(format!("{:?}", tmp_collection.join("")))
    } else {
        role_name_str
    }
}
