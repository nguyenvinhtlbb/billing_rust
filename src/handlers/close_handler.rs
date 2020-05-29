use crate::common::{BillingData, BillingHandler, ResponseError};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

pub struct CloseHandler {
    tx: Sender<u8>,
    stopped_flag: Arc<RwLock<bool>>,
}

impl CloseHandler {
    pub fn new(tx: Sender<u8>, stopped_flag: Arc<RwLock<bool>>) -> Self {
        CloseHandler { tx, stopped_flag }
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
                    println!("receiver dropped: {}", err);
                }
            }
        }
        Ok(response)
    }
}
