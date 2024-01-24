mod data_loader;
use data_loader::import_parquet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = import_parquet("data/kospi_tick.parquet")?;

    println!("{}", df.slice(0, 1000));

    Ok(())
}
