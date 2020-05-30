use billing_rust::cli;
use billing_rust::common::{BillConfig, Logger};
use billing_rust::log_message;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let log_file_path = exe_dir.join("log.log");
    let logger = Arc::new(Logger::new(log_file_path));
    let working_dir = env::current_dir().unwrap();
    let server_config = match BillConfig::load_from_file(exe_dir, &working_dir).await {
        Ok(value) => value,
        Err(err) => {
            log_message!(logger, Error, "load config error: {}", err);
            return;
        }
    };
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() > 1 {
        let command_str = cli_args.get(1).unwrap();
        if command_str == "stop" {
            cli::stop_server(server_config, logger).await;
            return;
        }
    }
    log_message!(
        logger,
        Info,
        "powered by liuguang @github https://github.com/liuguangw"
    );
    log_message!(
        logger,
        Info,
        "build by {}",
        option_env!("COMPILER_VERSION").unwrap_or("-")
    );
    log_message!(
        logger,
        Info,
        "Git Commit: {:.7}",
        option_env!("GIT_COMMIT_VERSION").unwrap_or("-")
    );
    cli::run_server(server_config, logger).await;
}
