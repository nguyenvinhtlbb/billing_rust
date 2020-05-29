use mysql_async::prelude::Queryable;
use mysql_async::Pool;
use mysql_async::{params, Row};

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
    id_card: Option<String>,
    point: i32,
}

macro_rules! account_from_row {
    ($row:ident,$($field_name:ident),*) => {
        Account{
        $(
            //id: row.get("id").unwrap(),
            $field_name:$row.get(stringify!($field_name)).unwrap(),
        )*
        }
    };
}

impl Account {
    pub async fn get_by_username(
        db_pool: &Pool,
        username: &str,
    ) -> Result<Option<Account>, mysql_async::error::Error> {
        let conn = db_pool.get_conn().await?;
        let (_, query_result) = conn
            .first_exec::<_, _, Row>(
                "SELECT * FROM account WHERE name=:name",
                params! {
                    "name" => username
                },
            )
            .await?;
        let account_info_option = if let Some(row) = query_result {
            Some(account_from_row!(
                row, id, name, password, question, answer, email, qq, id_card, point
            ))
        } else {
            None
        };
        Ok(account_info_option)
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
        conn.drop_exec("INSERT INTO account (name, password, question, email) VALUES (:name, :password, :question, :email)", params).await?;
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
        conn.drop_exec(
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
