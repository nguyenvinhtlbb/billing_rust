use mysql_async::params;
use mysql_async::prelude::Queryable;
use mysql_async::{Pool, Stmt};

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

impl Account {
    pub async fn get_by_username(
        db_pool: &Pool,
        username: &str,
    ) -> Result<Option<Account>, mysql_async::error::Error> {
        let conn = db_pool.get_conn().await?;
        let stm: Stmt<_> = conn
            .prepare("SELECT * FROM account WHERE name=:name")
            .await?;
        let query_result = stm
            .execute(params! {
                "name" => username
            })
            .await?;
        if query_result.is_empty() {
            return Ok(None);
        }
        let (_, mut rows) = query_result
            .map(|row| Account {
                id: row.get("id").unwrap(),
                name: row.get("name").unwrap(),
                password: row.get("password").unwrap(),
                question: row.get("question").unwrap(),
                answer: row.get("answer").unwrap(),
                email: row.get("email").unwrap(),
                qq: row.get("qq").unwrap(),
                point: row.get("point").unwrap(),
            })
            .await?;
        let account_info = rows.pop();
        Ok(account_info)
    }

    pub async fn insert_user(
        db_pool: &Pool,
        account_info: &Self,
    ) -> Result<(), mysql_async::error::Error> {
        let conn = db_pool.get_conn().await?;
        let params = params! {
            "name" => &account_info.name,
            "password" => &account_info.password,
            "question" => &account_info.question,
            "email" => &account_info.email
        };
        conn.batch_exec("INSERT INTO account (name, password, question, email) VALUES (:name, :password, :question, :email)", params).await?;
        Ok(())
    }

    pub async fn convert_point(
        username: &str,
        db_pool: &Pool,
        point: i32,
    ) -> Result<(), mysql_async::error::Error> {
        let conn = db_pool.get_conn().await?;
        let params = params! {
            "name" => username,
            "point" => point
        };
        conn.batch_exec(
            "UPDATE account SET point=point-:point WHERE name=:name",
            params,
        )
        .await?;
        Ok(())
    }

    pub fn new(username: &str, password: &str, super_password: &str, email: &str) -> Self {
        let mut account_info = Self::default();
        account_info.name = username.to_string();
        account_info.password = password.to_string();
        account_info.question = Some(super_password.to_string());
        account_info.email = Some(email.to_string());
        account_info
    }

    pub fn check_password(&self, input_password: &str) -> bool {
        self.password == input_password
    }

    //todo
    pub fn is_locked(&self) -> bool {
        false
    }

    pub fn point(&self) -> i32 {
        self.point
    }
}
