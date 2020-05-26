use super::BillConfigError;
use serde::Deserialize;
use std::path::Path;
use tokio::fs;

///配置信息
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct BillConfig {
    ip: String,
    port: i32,
    db_host: String,
    db_port: u16,
    db_user: String,
    db_password: String,
    db_name: String,
    auto_reg: bool,
    allow_ips: Vec<String>,
    transfer_number: i32,
}

impl Default for BillConfig {
    fn default() -> Self {
        BillConfig {
            ip: "127.0.0.1".to_string(),
            port: 12680,
            db_host: "127.0.0.1".to_string(),
            db_port: 3306,
            db_user: "root".to_string(),
            db_password: "root".to_string(),
            db_name: "web".to_string(),
            auto_reg: true,
            allow_ips: vec![],
            transfer_number: 1000,
        }
    }
}

impl BillConfig {
    /// 从配置文件中加载配置信息
    pub async fn load_from_file(
        exe_dir: &Path,
        working_dir: &Path,
    ) -> Result<BillConfig, BillConfigError> {
        let config_file_name = "config.json";
        let config_file_path = {
            // 配置默认目录为可执行文件所在目录
            let json_file_path = exe_dir.join(config_file_name);
            if json_file_path.is_file() {
                json_file_path
            } else {
                // 如果不存在则搜寻工作目录
                let backup_file_path = working_dir.join(config_file_name);
                if backup_file_path.is_file() {
                    backup_file_path
                } else {
                    //工作目录也不存在配置文件则抛出错误
                    let error_info = format!(
                        "config file {} not found !",
                        json_file_path.to_str().unwrap()
                    );
                    return Err(error_info.into());
                }
            }
        };
        //读取并解析配置文件
        let file_content = fs::read_to_string(config_file_path).await?;
        let config_value = serde_json::from_str(&file_content)?;
        Ok(config_value)
    }

    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn db_host(&self) -> &str {
        &self.db_host
    }

    pub fn db_port(&self) -> u16 {
        self.db_port
    }

    pub fn db_name(&self) -> &str {
        &self.db_name
    }

    pub fn db_user(&self) -> &str {
        &self.db_user
    }

    pub fn db_password(&self) -> &str {
        &self.db_password
    }
}
