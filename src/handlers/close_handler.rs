use crate::common::{BillingData, BillingHandler, ResponseError};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

pub struct CloseHandler {
    tx: Mutex<Sender<u8>>,
    stopped_flag: Arc<Mutex<bool>>,
}

impl CloseHandler {
    pub fn new(tx: Sender<u8>, stopped_flag: Arc<Mutex<bool>>) -> Self {
        CloseHandler {
            tx: Mutex::new(tx),
            stopped_flag,
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
            let mut stopped_flag_guard = self.stopped_flag.lock().await;
            *stopped_flag_guard = true;
            {
                let mut guard = self.tx.lock().await;
                if let Err(err) = guard.send(0).await {
                    println!("receiver dropped: {}", err);
                }
            }
        }
        Ok(response)
    }
}
