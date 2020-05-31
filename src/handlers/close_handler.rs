use crate::common::{BillingData, BillingHandler, LoggerSender, ResponseError};
use crate::log_message;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

pub struct CloseHandler {
    close_sender: Sender<u8>,
    stopped_flag: Arc<RwLock<bool>>,
    logger_sender: LoggerSender,
}

impl CloseHandler {
    pub fn new(
        close_sender: Sender<u8>,
        stopped_flag: Arc<RwLock<bool>>,
        logger_sender: LoggerSender,
    ) -> Self {
        CloseHandler {
            close_sender,
            stopped_flag,
            logger_sender,
        }
    }
}

#[async_trait]
impl BillingHandler for CloseHandler {
    fn get_type(&self) -> u8 {
        0
    }

    async fn get_response(&mut self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let mut response: BillingData = request.into();
        response.op_data.extend_from_slice(&[0x00, 0x00]);
        {
            let mut stopped_flag_guard = self.stopped_flag.write().await;
            *stopped_flag_guard = true;
            {
                if let Err(err) = self.close_sender.send(0).await {
                    log_message!(self.logger_sender, Error, "receiver dropped: {}", err);
                }
            }
        }
        Ok(response)
    }
}
