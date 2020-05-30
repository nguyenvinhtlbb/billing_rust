use tokio::sync::mpsc::Receiver;
use tokio::select;

/// 停止服务器的信号
pub(super) async fn shutdown_signal(mut rx: Receiver<u8>) -> i32 {
    //当任意一个Future触发
    select! {
        // Wait for the CTRL+C signal
         _ = tokio::signal::ctrl_c() => {
            1
         }
         // Wait for stop command
        _ = rx.recv() => {
            2
        }
    }
}