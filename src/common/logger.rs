use chrono::Local;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::join;
use tokio::sync::mpsc::Sender;

/// 日志工具
pub struct Logger {
    stdout: BufferWriter,
    stderr: BufferWriter,
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
            stdout: BufferWriter::stdout(choice),
            stderr: BufferWriter::stderr(choice),
            log_file,
        }
    }

    pub async fn log(&self, message_type: LogMessageType, message: &str) {
        let time_tag = format!("[{}]", Local::now().format("%F %T"));
        let message = format!(
            "[{}]: {}",
            match message_type {
                LogMessageType::Error => "error",
                LogMessageType::Warning => "warning",
                LogMessageType::Info => "info",
                LogMessageType::Debug => "debug",
            },
            message
        );
        let full_message = format!("{}{}\n", &time_tag, &message);
        join!(
            self.show_message(message_type, &time_tag, &message),
            self.log_to_file(&full_message)
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

    async fn show_message(&self, message_type: LogMessageType, time_tag: &str, message: &str) {
        let mut color_spec = ColorSpec::new();
        let buffer_writer = match message_type {
            LogMessageType::Error | LogMessageType::Warning => &self.stderr,
            _ => &self.stdout,
        };
        let mut buffer = buffer_writer.buffer();
        // 时间
        write!(&mut buffer, "{}", time_tag).unwrap();
        // 类型颜色
        match message_type {
            LogMessageType::Error => color_spec.set_fg(Some(Color::Red)),
            LogMessageType::Warning => color_spec.set_fg(Some(Color::Yellow)),
            LogMessageType::Info => color_spec.set_fg(Some(Color::Green)),
            LogMessageType::Debug => color_spec.set_fg(Some(Color::Cyan)),
        };
        if let Err(err) = buffer.set_color(&color_spec) {
            eprintln!("failed to set color: {}", err);
        }
        write!(&mut buffer, "{}", message).unwrap();
        //重置输出样式
        if let Err(err) = buffer.reset() {
            eprintln!("failed to reset color: {}", err);
        }
        //换行符
        writeln!(&mut buffer).unwrap();
        //输出
        if let Err(err) = buffer_writer.print(&buffer) {
            eprintln!("failed to output buffer: {}", err);
        }
    }
}

/// 用于日志发送的sender
pub type LoggerSender = Sender<(LogMessageType, String)>;
/// 输出日志
#[macro_export]
macro_rules! log_message {
    ($logger_sender:expr,$message_type:ident, $($args:tt)*) => {
        let message = format!($($args)*);
        if let Err(err) = $logger_sender.send(($crate::common::LogMessageType::$message_type,message)).await{
            eprintln!("logger dropped: {}",err);
        }
    };
}
