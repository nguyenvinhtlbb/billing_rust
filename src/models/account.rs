use sqlx::mysql::MySqlQueryAs;
use sqlx::FromRow;
use sqlx::MySqlPool;

/// 用户账号模型
#[derive(Debug, Default, FromRow)]
pub struct Account {
    id: i32,
    name: String,
    password: String,
    question: Option<String>,
    answer: Option<String>,
    email: Option<String>,
    qq: Option<String>,
    id_card: Option<String>,
    point: i32,
}

impl Account {
    pub async fn get_by_username(
        db_pool: &MySqlPool,
        username: &str,
    ) -> Result<Option<Account>, sqlx::Error> {
        let account_info_option = sqlx::query_as("SELECT * FROM account WHERE name=?")
            .bind(username)
            .fetch_optional(db_pool)
            .await?;
        Ok(account_info_option)
    }

    pub async fn insert_user(db_pool: &MySqlPool, account_info: &Self) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO account (name, password, question, email) VALUES (?,?,?,?)")
            .bind(&account_info.name)
            .bind(&account_info.password)
            .bind(&account_info.question)
            .bind(&account_info.email)
            .execute(db_pool)
            .await?;
        Ok(())
    }

    pub async fn convert_point(
        username: &str,
        db_pool: &MySqlPool,
        point: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE account SET point=point-? WHERE name=?")
            .bind(point)
            .bind(username)
            .execute(db_pool)
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

    pub fn is_locked(&self) -> bool {
        if let Some(ref value) = self.id_card {
            return value == "1";
        }
        false
    }

    pub fn point(&self) -> i32 {
        self.point
    }
}
