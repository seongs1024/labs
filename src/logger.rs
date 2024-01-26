use std::sync::{Arc, Mutex};
use tokio::{sync::mpsc::Receiver, task::JoinHandle};

#[derive(Debug)]
pub enum Event {
    OpenOrder(Side, String, i64, String, f64), //(Side, trader_id, time, code, quantity)
    Filled(Side, String, i64, String, f64, f64), //(Side, trader_id, time, code, quantity, price)
    Nav(String, f64),                          //(trader_id, nav)
}

#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

pub struct Logger {
    rx: Option<Receiver<Event>>,
}

impl Logger {
    pub fn new(rx: Receiver<Event>) -> Self {
        Self { rx: Some(rx) }
    }

    pub fn recv(&mut self) -> JoinHandle<()> {
        let Some(mut rx) = self.rx.take() else {
            todo!()
        };
        tokio::spawn(async move {
            while let Some(e) = rx.recv().await {
                println!("{:?}", e);
            }
        })
    }
}
