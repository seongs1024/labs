mod data_loader;
mod market;
mod trader;

use data_loader::import_parquet;
use market::MtBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = import_parquet("data/kospi_tick.parquet")?;
    // let df = df.slice(0, 10);

    let (mut market, traders) = MtBuilder::new(df, 200);

    for mut trader in traders {
        trader.recv();
    }

    market.send();

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    Ok(())
}
