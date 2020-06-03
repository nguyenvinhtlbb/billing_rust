use serde::export::Option::Some;
use std::env::Args;

/// cli命令类型
pub enum CliCommandType {
    /// 启动服务器(前台模式)
    RunServer,
    /// 启动服务器(后台模式)
    RunServerAtBackground,
    /// 停止服务器
    StopServer,
    // 命令行说明
    ShowUsage,
}

impl From<&mut Args> for CliCommandType {
    fn from(args_iter: &mut Args) -> Self {
        let first_command = args_iter.next();
        match first_command.as_deref() {
            Some("up") => {
                let command_extra_option = args_iter.next();
                if let Some("-d") = command_extra_option.as_deref() {
                    return CliCommandType::RunServerAtBackground;
                }
                CliCommandType::RunServer
            }
            Some("stop") => CliCommandType::StopServer,
            _ => CliCommandType::ShowUsage,
        }
    }
}
