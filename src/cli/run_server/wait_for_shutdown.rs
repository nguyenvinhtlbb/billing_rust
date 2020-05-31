#[cfg(unix)]
use signal::unix::{self, SignalKind};
#[cfg(unix)]
use std::io;
use tokio::select;
use tokio::signal;
use tokio::sync::mpsc::Receiver;

/// 等待停止服务器
pub(super) async fn wait_for_shutdown(mut close_receiver: Receiver<u8>) -> i32 {
    #[cfg(unix)]
    let exit_signal_future = unix_exit_signal();
    #[cfg(not(unix))]
    let exit_signal_future = signal::ctrl_c();
    // 等待系统发送给程序的停止信号,或者billing stop命令的到来
    select! {
        // Wait for exit signal
         _ = exit_signal_future => {
            1
         }
         // Wait for stop command
        _ = close_receiver.recv() => {
            2
        }
    }
}

/// SIGQUIT
#[cfg(unix)]
async fn unix_quit_signal() -> io::Result<()> {
    unix::signal(SignalKind::quit())?.recv().await;
    Ok(())
}

/// SIGTERM
#[cfg(unix)]
async fn unix_terminate_signal() -> io::Result<()> {
    unix::signal(SignalKind::terminate())?.recv().await;
    Ok(())
}

#[cfg(unix)]
async fn unix_exit_signal() -> io::Result<()> {
    select! {
        value = signal::ctrl_c() => {
            value
         }
         value = unix_quit_signal() => {
            value
         }
         value = unix_terminate_signal() => {
            value
         }
    }
}
