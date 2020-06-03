use billing_rust::common::{BillConfig, CliCommandType};
use billing_rust::log_message;
use billing_rust::{cli, services};
use std::env;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

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
    let mut cli_args = env::args();
    let exe_path = cli_args.next().unwrap();
    match CliCommandType::from(&mut cli_args) {
        CliCommandType::StopServer => {
            //停止服务
            cli::stop_server(server_config, logger_sender).await;
            logger_service.await.unwrap();
        }
        CliCommandType::RunServer => {
            //启动服务器(前台模式)
            cli::run_server(server_config, logger_sender).await;
            tokio::select! {
                //wait for logger service stopped
                _=logger_service=>{
                },
                //or timeout force stop
                _=time::delay_for(Duration::from_millis(900)) =>{
                }
            }
        }
        CliCommandType::RunServerAtBackground => {
            //启动服务器(后台模式)
            cli::run_background(&exe_path, logger_sender).await;
            logger_service.await.unwrap();
        }
        CliCommandType::ShowUsage => {
            //命令行说明
            cli::show_usage();
            drop(logger_sender);
            logger_service.await.unwrap();
        }
    };
}
