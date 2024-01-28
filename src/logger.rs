use std::sync::{Arc, Mutex};
use tokio::{sync::mpsc::Receiver, task::JoinHandle};

#[derive(Debug)]
pub enum Event {
    OpenOrder(Side, String, String, i64, String, i64), //(Side, trader_name, strategy_name, time, code, quantity)
    Filled(Side, String, String, i64, String, f64, i64), //(Side, trader_name, strategy_name, time, code, quantity, price)
    Nav(String, String, i64, f64),                       //(trader_name, strategy_name, time, nav)
}

#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

pub struct Logger {
    rx: Option<Receiver<Event>>,
    rerun: rerun::RecordingStream,
}

impl Logger {
    pub fn new(rx: Receiver<Event>, rerun: rerun::RecordingStream) -> Self {
        Self {
            rx: Some(rx),
            rerun,
        }
    }

    pub fn recv(&mut self) -> JoinHandle<()> {
        let Some(mut rx) = self.rx.take() else {
            todo!()
        };
        let rec = self.rerun.clone();
        tokio::spawn(async move {
            while let Some(e) = rx.recv().await {
                match e {
                    Event::OpenOrder(side, trader_name, strategy_name, time, code, quantity) => {
                        rec.set_time_nanos("order", time * 1_000);
                        rec.log(
                            format!("order/{:?}/strategy_{}/{}", side, strategy_name, trader_name),
                            &rerun::TextLog::new(format!("{:?} {}, q: {}", side, code, quantity)),
                        )
                        .unwrap();
                    }
                    Event::Nav(trader_name, strategy_name, time, nav) => {
                        rec.set_time_nanos("nav", time * 1_000);
                        rec.log(
                            format!("nav/strategy_{}/{}", strategy_name, trader_name),
                            &rerun::TimeSeriesScalar::new(nav),
                        )
                        .unwrap();
                    }
                    e => {
                        println!("{:?}", e);
                    }
                };
            }
        })
    }
}
