use crate::models::Account;
use mysql_async::Pool;

pub async fn get_register_result(
    db_pool: &Pool,
    username: &str,
    password: &str,
    super_password: &str,
    email: &str,
) -> Result<u8, mysql_async::error::Error> {
    let account_info = Account::get_by_username(db_pool, username).await?;
    let reg_err_code = 4u8;
    //用户账号已存在
    if account_info.is_some() {
        return Ok(reg_err_code);
    }
    if email == "1@1.com" {
        //不允许默认的邮箱
        return Ok(reg_err_code);
    }
    let account_info = Account::new(username, password, super_password, email);
    Account::insert_user(db_pool, &account_info).await?;
    Ok(1)
}
