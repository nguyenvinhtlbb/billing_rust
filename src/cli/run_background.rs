use crate::common::LoggerSender;
use crate::log_message;
use std::process::Stdio;
use tokio::process::Command;

/// 使程序在后台运行
pub async fn run_background(exe_path: &str, mut logger_sender: LoggerSender) {
    let mut command = Command::new(exe_path);
    command.args(&["up"]).stdout(Stdio::null());
    #[cfg(windows)]
    {
        // for windows
        // CREATE_BREAKAWAY_FROM_JOB + CREATE_NO_WINDOW
        let flag = 0x0100_0000 | 0x0800_0000;
        command.creation_flags(flag);
    }
    if let Err(err) = command.spawn() {
        log_message!(
            logger_sender,
            Error,
            "failed to spawn child process: {}",
            err
        );
    }
}
