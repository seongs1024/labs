use crate::{market::{MarketEvent, Tick}, strategy::Strategy,};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{error::RecvError, Receiver};

pub struct Trader {
    name: Arc<String>,
    rx: Option<Receiver<MarketEvent>>,
    strategy: Option<Box<dyn Strategy + Send>>,
}

impl Trader {
    pub fn new(name: String, rx: Receiver<MarketEvent>) -> Self {
        Self {
            name: Arc::new(name),
            rx: Some(rx),
            strategy: None,
        }
    }

    pub fn add_strategy<S>(&mut self, strategy: S)
    where
        S: Strategy + Send + 'static
    {
        self.strategy = Some(Box::new(strategy));
    }

    pub fn recv(&mut self) {
        let name = self.name.clone();
        let Some(mut rx) = self.rx.take() else {
            todo!()
        };
        let Some(mut strategy) = self.strategy.take() else {
            todo!()
        };
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(tick) => {
                        println!("{}: {:?}", name, tick);
                        // strategy.signal(tick);
                    },
                    // Ok(MarketEvent::Tick(&tick)) => {
                    //     println!("{}: {:?}", name, tick);
                    //     // strategy.signal(tick);
                    // },
                    // Ok(MarketEvent::SecCodes(&sec_codes)) => {
                    //     println!("{}: {:?}", name, sec_codes);
                    //     // strategy.update_sec_codes(sec_codes);
                    // },
                    Err(RecvError::Lagged(behind)) => {
                        eprintln!("{}: lagged behind {}", name, behind)
                    }
                    Err(RecvError::Closed) => break,
                }
            }
        });
    }
}
