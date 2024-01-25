use polars::prelude::*;
use std::ops::{Add, Div, Mul, Rem};

pub fn import_parquet<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(path)?;
    let df = ParquetReader::new(&mut file).finish()?;

    let df = df
        .lazy()
        .select([
            convert_trade_time_to_elapsed_micros(col("trade_time")),
            cols(["sec_code", "trade_price"]),
        ])
        .sort("trade_time", Default::default())
        .collect()?;
    Ok(df)
}

// convert "trade_time"
//    from: HH(3600_secs)mm(60_secs)ss000000(timeunit: micro seconds)
//    to: elapsed micro seconds.
// HH -> micro seconds
// mm -> micro seconds
// ss000000
fn convert_trade_time_to_elapsed_micros(col: Expr) -> Expr {
    hours_to_micros(col.clone())
        .add(mins_to_micros(col.clone()))
        .add(col.rem(lit(100_000_000)))
}

fn hours_to_micros(col: Expr) -> Expr {
    col.div(lit(10_000_000_000i64))
        .rem(lit(100))
        .mul(lit(3_600_000_000i64))
}

fn mins_to_micros(col: Expr) -> Expr {
    col.div(lit(100_000_000)).rem(lit(100)).mul(lit(60_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Sub;

    #[tokio::test]
    async fn validate_ticks() -> Result<(), Box<dyn std::error::Error>> {
        let df = import_parquet("data/kospi_tick.parquet")?;
        println!("data: {}", df);

        let low_price = df
            .clone()
            .lazy()
            .filter(col("trade_price").lt(lit(10)))
            .collect()?;
        println!("price lower than 10: {}", low_price);

        let start_end = df
            .clone()
            .lazy()
            .group_by([col("sec_code")])
            .agg([
                col("trade_time").first().alias("first_time"),
                col("trade_time").last().alias("last_time"),
            ])
            .with_column(col("last_time").sub(col("first_time")).alias("duration"))
            .sort("first_time", Default::default())
            .collect()?;
        println!("group by sec_code: {}", start_end);

        Ok(())
    }
}
