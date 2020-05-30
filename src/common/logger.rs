use chrono::Local;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::join;
use tokio::sync::Mutex;

/// 日志工具
pub struct Logger {
    stdout: Mutex<StandardStream>,
    stderr: Mutex<StandardStream>,
    log_file: PathBuf,
}

/// 日志类型
pub enum LogMessageType {
    Error,
    Warning,
    Info,
    Debug,
}

impl Logger {
    pub fn new(log_file: PathBuf) -> Self {
        let choice = ColorChoice::Auto;
        Logger {
            stdout: Mutex::new(StandardStream::stdout(choice)),
            stderr: Mutex::new(StandardStream::stderr(choice)),
            log_file,
        }
    }

    pub async fn log(&self, message_type: LogMessageType, message: &str) {
        let time_now = Local::now();
        let message = &format!(
            "[{}][{}]: {}\n",
            time_now.format("%F %T"),
            match message_type {
                LogMessageType::Error => "error",
                LogMessageType::Warning => "warning",
                LogMessageType::Info => "info",
                LogMessageType::Debug => "debug",
            },
            message
        );
        join!(
            self.show_message(message_type, message),
            self.log_to_file(message)
        );
    }

    async fn log_to_file(&self, message: &str) {
        let mut log_file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.log_file)
            .await
        {
            Ok(value) => value,
            Err(err) => {
                eprintln!(
                    "open file {} error: {}",
                    self.log_file.to_str().unwrap(),
                    err
                );
                return;
            }
        };
        if let Err(err) = log_file.write_all(message.as_bytes()).await {
            eprintln!(
                "failed to write to {},err={}",
                self.log_file.as_path().to_str().unwrap_or("<file>"),
                err
            )
        }
    }

    async fn show_message(&self, message_type: LogMessageType, message: &str) {
        let mut color = ColorSpec::new();
        let stream_guard = match message_type {
            LogMessageType::Error => {
                color.set_fg(Some(Color::Red));
                &self.stderr
            }
            LogMessageType::Warning => {
                color.set_fg(Some(Color::Yellow));
                &self.stderr
            }
            LogMessageType::Info => {
                color.set_fg(Some(Color::Green));
                &self.stdout
            }
            LogMessageType::Debug => {
                color.set_fg(Some(Color::Cyan));
                &self.stdout
            }
        };
        let mut stream_guard = stream_guard.lock().await;
        if let Err(err) = stream_guard.set_color(&color) {
            eprintln!("failed to set color: {}", err);
        }
        stream_guard.write_all(message.as_bytes()).unwrap();
        stream_guard.reset().unwrap();
    }
}

/// 输出日志
#[macro_export]
macro_rules! log_message {
    ($logger:expr,$message_type:ident, $($args:tt)*) => {
        let message = format!($($args)*);
        $logger.log($crate::common::LogMessageType::$message_type,&message).await;
    };
}
