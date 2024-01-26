use std::sync::{Arc, Mutex};
use tokio::{sync::mpsc::Receiver, task::JoinHandle};

#[derive(Debug)]
pub enum Event {
    Buy,
    Sell,
    Nav,
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
                println!("got = {:?}", e);
            }
        })
    }
}
