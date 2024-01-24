use crate::data_loader::import_parquet;

use polars::prelude::*;

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
