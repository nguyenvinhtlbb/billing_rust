use tokio::select;
use tokio::sync::mpsc::Receiver;

/// 停止服务器的信号
pub(super) async fn shutdown_signal(mut close_receiver: Receiver<u8>) -> i32 {
    //当任意一个Future触发
    select! {
        // Wait for the CTRL+C signal
         _ = tokio::signal::ctrl_c() => {
            1
         }
         // Wait for stop command
        _ = close_receiver.recv() => {
            2
        }
    }
}
