use crate::common::{BillingData, BillingHandler, LoggerSender, ResponseError};
use crate::log_message;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, RwLock};

pub struct CloseHandler {
    tx: Sender<u8>,
    stopped_flag: Arc<RwLock<bool>>,
    logger_sender: Mutex<LoggerSender>,
}

impl CloseHandler {
    pub fn new(
        tx: Sender<u8>,
        stopped_flag: Arc<RwLock<bool>>,
        logger_sender: LoggerSender,
    ) -> Self {
        CloseHandler {
            tx,
            stopped_flag,
            logger_sender: Mutex::new(logger_sender),
        }
    }
}

#[async_trait]
impl BillingHandler for CloseHandler {
    fn get_type(&self) -> u8 {
        0
    }

    async fn get_response(&self, request: &BillingData) -> Result<BillingData, ResponseError> {
        let mut response: BillingData = request.into();
        response.op_data.extend_from_slice(&[0x00, 0x00]);
        {
            let mut stopped_flag_guard = self.stopped_flag.write().await;
            *stopped_flag_guard = true;
            {
                let mut sender = self.tx.clone();
                if let Err(err) = sender.send(0).await {
                    let mut logger_sender = self.logger_sender.lock().await;
                    log_message!(logger_sender, Error, "receiver dropped: {}", err);
                }
            }
        }
        Ok(response)
    }
}
