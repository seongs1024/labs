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
                    Event::OpenOrder(side, trader_name, time, code, quantity) => {
                        rec.set_time_nanos("time", time * 1_000);
                        rec.log(
                            format!("strategy/{}", trader_name),
                            &rerun::TimeSeriesScalar::new((time as f64).sin())
                                .with_label(format!("{:?} {}", side, code)),
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
