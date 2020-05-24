use billing_rust::models::BillConfig;
use std::env;

#[tokio::main]
async fn main() {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let working_dir = env::current_dir().unwrap();
    let app_config = match BillConfig::load_from_file(exe_dir, &working_dir).await {
        Ok(value) => value,
        Err(err) => {
            eprintln!("load config error: {}", err);
            return;
        }
    };
    dbg!(&app_config);
    println!("Hello, world!");
}
