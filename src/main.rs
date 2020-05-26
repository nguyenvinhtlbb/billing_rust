use billing_rust::common::BillConfig;
use billing_rust::cli;
use std::env;

#[tokio::main]
async fn main() {
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
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() > 1 {
        let command_str = cli_args.get(1).unwrap();
        if command_str == "stop" {
            //do stop
            return;
        }
    }
    cli::run_server(server_config).await;
}
