use crate::market::MarketEvent;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{error::RecvError, Receiver};

pub struct Trader {
    name: Arc<String>,
    rx: Option<Receiver<MarketEvent>>,
}

impl Trader {
    pub fn new(name: String, rx: Receiver<MarketEvent>) -> Self {
        Self {
            name: Arc::new(name),
            rx: Some(rx),
        }
    }

    pub fn recv(&mut self) {
        let name = self.name.clone();
        let Some(mut rx) = self.rx.take() else {
            todo!()
        };
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(market_data) => println!("{}: {:?}", name, market_data),
                    Err(RecvError::Lagged(behind)) => {
                        eprintln!("{}: lagged behind {}", name, behind)
                    }
                    Err(RecvError::Closed) => break,
                }
            }
        });
    }
}
