use tokio::select;
use tokio::sync::mpsc::Receiver;

/// 等待停止服务器
pub(super) async fn wait_for_shutdown(mut close_receiver: Receiver<u8>) -> i32 {
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
