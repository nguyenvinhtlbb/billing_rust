use crate::models::Account;
use mysql_async::Pool;

pub async fn get_login_result(
    db_pool: &Pool,
    username: &str,
    password: &str,
) -> Result<u8, mysql_async::error::Error> {
    let account_info = Account::get_account_by_username(db_pool, username).await?;
    //用户账号不存在
    if account_info.is_none() {
        return Ok(9);
    }
    let account_info = account_info.unwrap();
    //dbg!(&account_info);
    if !account_info.check_password(password) {
        return Ok(3);
    }
    Ok(1)
}
