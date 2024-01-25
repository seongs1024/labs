mod data_loader;
mod market;

use data_loader::import_parquet;
use market::MsBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = import_parquet("data/kospi_tick.parquet")?;
    // let df = df.slice(0, 10);

    let (mut market, strategies) = MsBuilder::new(df, 200);

    for mut strategy in strategies {
        strategy.recv();
    }

    market.send();

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    Ok(())
}
