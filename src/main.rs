use billing_rust::common::BillConfig;
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
    let cli_args: Vec<String> = env::args().collect();
    let arg_length = cli_args.len();
    let first_command = if arg_length > 1 {
        cli_args[1].as_str()
    } else {
        ""
    };
    match first_command {
        "stop" => {
            //停止服务的命令
            cli::stop_server(server_config, logger_sender).await;
            logger_service.await.unwrap();
        }
        "up" => {
            let run_at_background = if arg_length > 2 {
                let extra_command = &cli_args[2];
                extra_command == "-d"
            } else {
                false
            };
            if run_at_background {
                cli::run_background(&cli_args[0], logger_sender).await;
                logger_service.await.unwrap();
            } else {
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
        }
        _ => {
            cli::show_usage();
            drop(logger_sender);
            logger_service.await.unwrap();
        }
    };
}
