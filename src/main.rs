use billing_rust::common::BillConfig;
use billing_rust::log_message;
use billing_rust::{cli, services};
use std::env;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    //获取文件路径信息
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let working_dir = env::current_dir().unwrap();
    //加载logger服务
    let (mut logger_sender, logger_receiver) = mpsc::channel(20);
    let logger_service = tokio::spawn(services::logger_service(
        exe_dir.to_path_buf(),
        logger_receiver,
    ));
    //加载配置信息
    let server_config = match BillConfig::load_from_file(exe_dir, &working_dir).await {
        Ok(value) => value,
        Err(err) => {
            log_message!(logger_sender, Error, "load config error: {}", err);
            drop(logger_sender);
            logger_service.await.unwrap();
            return;
        }
    };
    //dbg!(&server_config);
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() > 1 {
        let command_str = cli_args.get(1).unwrap();
        if command_str == "stop" {
            cli::stop_server(server_config, logger_sender).await;
            logger_service.await.unwrap();
            return;
        }
    }
    cli::run_server(server_config, logger_sender).await;
    logger_service.await.unwrap();
}
