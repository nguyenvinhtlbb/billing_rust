use billing_rust::common::BillConfig;
use billing_rust::{cli, services};
use std::env;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    //加载配置
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let working_dir = env::current_dir().unwrap();
    let server_config = match BillConfig::load_from_file(exe_dir, &working_dir).await {
        Ok(value) => value,
        Err(err) => {
            eprintln!("load config error: {}", err);
            return;
        }
    };
    //dbg!(&server_config);
    //加载logger服务
    let (logger_sender, logger_receiver) = mpsc::channel(20);
    tokio::spawn(services::logger_service(
        exe_dir.to_path_buf(),
        logger_receiver,
    ));
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() > 1 {
        let command_str = cli_args.get(1).unwrap();
        if command_str == "stop" {
            cli::stop_server(server_config, logger_sender).await;
            return;
        }
    }
    cli::run_server(server_config, logger_sender).await;
}
