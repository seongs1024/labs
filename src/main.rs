mod data_loader;
mod market;
mod mt_builder;
mod trader;
mod strategy;

use data_loader::import_parquet;
use mt_builder::MtBuilder;
use strategy::{StrategyA};

#[tokio::main(flavor = "multi_thread", worker_threads = 220)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = import_parquet("data/kospi_tick.parquet")?;
    // let df = df.slice(0, 10);

    let (mut market, traders) = MtBuilder::new(df, 200);

    for mut trader in traders {
        trader.add_strategy(StrategyA::new());
        trader.recv();
    }

    let handle = market.send();

    tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;
    handle.abort();
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    Ok(())
}
