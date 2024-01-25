use polars::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::{
    sync::broadcast::{self, error::RecvError, Receiver, Sender},
    time::{sleep, Duration, Instant},
};

// struct MarketData {
//     idx: usize,
//     time: i64,
//     code: String,
//     price: f64,
// }

type Itcp = (usize, i64, String, f64);

pub struct Market<MarketData> {
    df: Arc<DataFrame>,
    tx: Option<Sender<MarketData>>,
}

impl Market<Itcp> {
    pub fn new(df: DataFrame, tx: Sender<Itcp>) -> Self {
        Self {
            df: Arc::new(df),
            tx: Some(tx),
        }
    }

    pub fn send(&mut self) {
        let df = self.df.clone();
        let Some(tx) = self.tx.take() else { todo!() };
        tokio::spawn(async move {
            let time_offset = 9 * 3_600_000_000i64;
            // simulation_start and idx need to be initialized out of time_offset
            let simulation_start = Instant::now();
            let mut idx = 0;
            loop {
                if let Some(tick) = df.get(idx) {
                    let AnyValue::Int64(time) = tick[0] else {
                        todo!()
                    };
                    let AnyValue::String(code) = tick[1] else {
                        todo!()
                    };
                    let AnyValue::Float64(price) = tick[2] else {
                        todo!()
                    };

                    // Self::wait_until(&time, &simulation_start, &time_offset).await;
                    // TODO: when backtests from the middle of opening markets(e.g. backtests from 1 p.m.)
                    let simulation_duration = Instant::now() - simulation_start;
                    let real_time = time - time_offset;
                    if real_time < 0 {
                        continue;
                    }
                    let real_time = Duration::from_micros(real_time as u64);
                    sleep(real_time.saturating_sub(simulation_duration)).await;
                    match tx.send((idx, time, code.to_string(), price)) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{}", e);
                            break;
                        }
                    };
                } else {
                    break;
                }
                idx += 1;
            }
        });
    }

    // async fn wait_until(time: &i64, simulation_start: &Instant, time_offset: &i64) {
    // }
}

pub struct Strategy<MarketData> {
    name: Arc<String>,
    rx: Option<Receiver<MarketData>>,
}

impl Strategy<Itcp> {
    pub fn new(name: String, rx: Receiver<Itcp>) -> Self {
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

// Market and Strategies Builder
pub struct MsBuilder;

impl MsBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(df: DataFrame, strategies: usize) -> (Market<Itcp>, Vec<Strategy<Itcp>>) {
        let (tx, rx) = broadcast::channel(1_000);

        if strategies < 2 {
            todo!();
        }
        let strategies: Vec<_> = (0..(strategies - 1))
            .map(|_| tx.subscribe())
            .chain(std::iter::once(rx))
            .enumerate()
            .map(|(i, rx)| Strategy::new(format!("{}", i), rx))
            .collect();
        let market = Market::new(df, tx);
        (market, strategies)
    }
}
