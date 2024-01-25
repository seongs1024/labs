use polars::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::{
    sync::broadcast::Sender,
    task::JoinHandle,
    time::{sleep, Duration, Instant},
};

#[derive(Clone, Debug)]
pub enum MarketEvent {
    Tick(usize, i64, String, f64),
    SecCodes(Vec<String>),
}

pub struct Market {
    df: Arc<DataFrame>,
    tx: Option<Sender<MarketEvent>>,
}

impl Market {
    pub fn new(df: DataFrame, tx: Sender<MarketEvent>) -> Self {
        Self {
            df: Arc::new(df),
            tx: Some(tx),
        }
    }

    pub fn send(&mut self) -> JoinHandle<()> {
        let df = self.df.clone();
        let Some(tx) = self.tx.take() else { todo!() };
        tokio::spawn(async move {
            let mut sec_codes: HashMap<String, (i64, f64)> = HashMap::new();
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

                    // TODO: check available securities. need to deal with circuit breakers
                    // sec_codes.insert(code.to_owned(), (time, price));
                    // if sec_codes.iter().any(|(_, (t, _))| time - t > 1_000_000) {
                    //     sec_codes = sec_codes
                    //         .into_iter()
                    //         .filter(|(_, (t, _))| time - t <= 1_000_000)
                    //         .collect();
                    //     match tx.send(MarketEvent::SecCodes(sec_codes
                    //             .iter()
                    //             .map(|(code, (_, _))| code.to_owned())
                    //             .collect()
                    //         )) {
                    //         Ok(_) => {}
                    //         Err(e) => {
                    //             eprintln!("{}", e);
                    //             break;
                    //         }
                    //     };
                    // }

                    // Self::wait_until(&time, &simulation_start, &time_offset).await;
                    // TODO: when backtests from the middle of opening markets(e.g. backtests from 1 p.m.)
                    let simulation_duration = Instant::now() - simulation_start;
                    let real_time = time - time_offset;
                    if real_time < 0 {
                        continue;
                    }
                    let real_time = Duration::from_micros(real_time as u64);
                    sleep(real_time.saturating_sub(simulation_duration)).await;
                    match tx.send(MarketEvent::Tick(idx, time, code.to_owned(), price)) {
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
        })
    }

    // async fn wait_until(time: &i64, simulation_start: &Instant, time_offset: &i64) {
    // }
}
