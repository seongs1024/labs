use polars::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::{
    sync::{broadcast::Sender, RwLock},
    task::JoinHandle,
    time::{sleep, Duration, Instant},
};

#[derive(Clone, Debug)]
pub enum MarketEvent {
    Tick(Tick),
}

#[derive(Clone, Debug)]
pub struct Tick {
    pub idx: usize,
    pub time: i64,
    pub code: String,
    pub price: f64,
}

pub type Securities = Arc<RwLock<HashMap<String, f64>>>;

pub struct Market {
    df: Option<Arc<DataFrame>>,
    tx: Option<Sender<MarketEvent>>,
    rerun: rerun::RecordingStream,
    sec_codes: Securities,
}

impl Market {
    pub fn new(
        tx: Sender<MarketEvent>,
        rerun: rerun::RecordingStream,
        sec_codes: Securities,
    ) -> Self {
        Self {
            df: None,
            tx: Some(tx),
            rerun,
            sec_codes,
        }
    }

    pub fn add_ticks(&mut self, df: DataFrame) {
        self.df = Some(Arc::new(df));
    }

    pub fn send(&mut self) -> JoinHandle<()> {
        let Some(df) = self.df.clone().take() else {
            todo!()
        };
        let Some(tx) = self.tx.take() else { todo!() };
        let sec_codes = self.sec_codes.clone();
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

                    let tick = Tick {
                        idx,
                        time,
                        code: code.to_owned(),
                        price,
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
                    {
                        let mut sc = sec_codes.write().await;
                        (*sc).insert(code.to_owned(), price);
                    }
                    match tx.send(MarketEvent::Tick(tick)) {
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

    pub fn is_ok(&self) -> bool {
        self.df.is_some() && self.tx.is_some()
    }
}
