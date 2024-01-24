mod data_loader;
mod market;

use data_loader::import_parquet;

use polars::prelude::*;
use tokio::{
    sync::{
        broadcast::{self, error::RecvError},
        mpsc, watch,
    },
    time::{sleep, sleep_until, Duration},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = import_parquet("data/kospi_tick.parquet")?;
    // let df = df.slice(0, 1_000);

    let start = 9 * 3_600_000_000i64;
    let (tx, rx) = broadcast::channel(1_000);
    let rxs: Vec<_> = (0..200)
        .map(|_| tx.subscribe())
        .chain(std::iter::once(rx))
        .enumerate()
        .map(|(i, mut rx)| {
            tokio::spawn(async move {
                loop {
                    match rx.recv().await {
                        Ok(itcp) => println!("{}: {:?}", i, itcp),
                        Err(RecvError::Lagged(behind)) => {
                            eprintln!("{}: lagged behind {}", i, behind)
                        }
                        Err(RecvError::Closed) => break,
                    }
                }
            })
        })
        .collect();

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

            sleep(Duration::from_micros(100)).await;
            match tx.send(Some((idx, time, code.to_string(), price))) {
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
    Ok(())
}
