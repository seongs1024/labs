use crate::{
    logger::Event,
    market::{MarketEvent, Securities},
    strategy::Strategy,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    broadcast::{self, error::RecvError},
    mpsc,
};

pub struct Trader {
    name: Arc<String>,
    rx: Option<broadcast::Receiver<MarketEvent>>,
    log_tx: Option<mpsc::Sender<Event>>,
    strategy: Option<Strategy>,
    rerun: rerun::RecordingStream,
    sec_codes: Securities,
}

impl Trader {
    pub fn new(
        name: String,
        rx: broadcast::Receiver<MarketEvent>,
        log_tx: mpsc::Sender<Event>,
        rerun: rerun::RecordingStream,
        sec_codes: Securities,
    ) -> Self {
        Self {
            name: Arc::new(name),
            rx: Some(rx),
            log_tx: Some(log_tx),
            strategy: None,
            rerun,
            sec_codes,
        }
    }

    pub fn add_strategy(&mut self, strategy: Strategy) {
        self.strategy = Some(strategy);
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
        let sec_codes = self.sec_codes.clone();
        tokio::spawn(async move {
            let mut stocks_held: HashMap<String, i64> = HashMap::new();
            let mut cash = strategy.config.start_balance;
            let mut nav = cash;

            loop {
                match rx.recv().await {
                    Ok(MarketEvent::Tick(tick)) => {
                        nav += match stocks_held.get(&tick.code) {
                            Some(quantity) => (*quantity as f64) * tick.price_change,
                            None => 0.0,
                        };
                        log_tx
                            .send(Event::Nav(
                                name.to_string(),
                                strategy.config.name.to_owned(),
                                tick.time,
                                nav,
                            ))
                            .await;

                        match strategy.buy_signal(&tick, &name, cash, &sec_codes).await {
                            Some(event) => {
                                if let Event::OpenOrder(
                                    ref side,
                                    ref trader_name,
                                    ref strategy_name,
                                    time,
                                    ref code,
                                    quantity,
                                ) = event
                                {
                                    stocks_held
                                        .entry(code.to_owned())
                                        .and_modify(|held| *held += quantity)
                                        .or_insert(quantity);
                                    cash -= {
                                        let sec_codes = sec_codes.read().await;
                                        *(*sec_codes).get(code).unwrap() * (quantity as f64)
                                    };
                                    log_tx.send(event).await;
                                }
                            }
                            _ => {}
                        };
                        match strategy.sell_signal(&tick, &name, &sec_codes).await {
                            Some(event) => {
                                log_tx.send(event).await;
                            }
                            _ => {}
                        };
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
