use crate::{logger::Event, market::MarketEvent, strategy::Strategy};
use std::sync::Arc;
use tokio::sync::{
    broadcast::{self, error::RecvError},
    mpsc,
};

pub struct Trader {
    name: Arc<String>,
    rx: Option<broadcast::Receiver<MarketEvent>>,
    log_tx: Option<mpsc::Sender<Event>>,
    strategy: Option<Box<dyn Strategy + Send>>,
}

impl Trader {
    pub fn new(
        name: String,
        rx: broadcast::Receiver<MarketEvent>,
        log_tx: mpsc::Sender<Event>,
    ) -> Self {
        Self {
            name: Arc::new(name),
            rx: Some(rx),
            log_tx: Some(log_tx),
            strategy: None,
        }
    }

    pub fn add_strategy<S>(&mut self, strategy: S)
    where
        S: Strategy + Send + 'static,
    {
        self.strategy = Some(Box::new(strategy));
    }

    pub fn recv(&mut self) {
        let name = self.name.clone();
        let Some(mut rx) = self.rx.take() else {
            todo!()
        };
        let Some(log_tx) = self.log_tx.take() else {
            todo!()
        };
        let Some(mut strategy) = self.strategy.take() else {
            todo!()
        };
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(MarketEvent::Tick(tick)) => {
                        // println!("{}: {:?}", name, tick);
                        match strategy.buy_signal(&tick, &name) {
                            Some(event) => {
                                log_tx.send(event).await;
                            }
                            _ => {}
                        };
                        match strategy.sell_signal(&tick, &name) {
                            Some(event) => {
                                log_tx.send(event).await;
                            }
                            _ => {}
                        };
                    }
                    Ok(MarketEvent::SecCodes(sec_codes)) => {
                        // println!("{}: {:?}", name, sec_codes);
                        strategy.update_sec_codes(sec_codes);
                    }
                    Err(RecvError::Lagged(behind)) => {
                        eprintln!("{}: lagged behind {}", name, behind)
                    }
                    Err(RecvError::Closed) => break,
                }
            }
        });
    }

    pub fn is_ok(&self) -> bool {
        self.rx.is_some() && self.strategy.is_some()
    }
}
