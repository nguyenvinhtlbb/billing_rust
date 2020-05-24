/// 用户账号模型
#[derive(Debug, Default)]
pub struct Account {
    id: i32,
    name: String,
    password: String,
    question: Option<String>,
    answer: Option<String>,
    email: Option<String>,
    qq: Option<String>,
    point: i32,
}
