use billing_rust::common::BillConfig;
use billing_rust::{cli, log_message, services};
use std::env;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let (mut logger_sender, logger_receiver) = mpsc::channel(20);
    tokio::spawn(services::logger_service(
        exe_dir.to_path_buf(),
        logger_receiver,
    ));
    let working_dir = env::current_dir().unwrap();
    let server_config = match BillConfig::load_from_file(exe_dir, &working_dir).await {
        Ok(value) => value,
        Err(err) => {
            log_message!(logger_sender, Error, "load config error: {}", err);
            return;
        }
    };
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() > 1 {
        let command_str = cli_args.get(1).unwrap();
        if command_str == "stop" {
            cli::stop_server(server_config, logger_sender).await;
            return;
        }
    }
    log_message!(
        logger_sender,
        Info,
        "powered by liuguang @github https://github.com/liuguangw"
    );
    log_message!(
        logger_sender,
        Info,
        "build by {}",
        option_env!("COMPILER_VERSION").unwrap_or("-")
    );
    log_message!(
        logger_sender,
        Info,
        "Git Commit: {:.7}",
        option_env!("GIT_COMMIT_VERSION").unwrap_or("-")
    );
    cli::run_server(server_config, logger_sender).await;
}
