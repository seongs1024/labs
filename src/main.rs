mod data_loader;
mod logger;
mod market;
mod simulation_builder;
mod strategy;
mod trader;

use data_loader::import_parquet;
use simulation_builder::SimulationBuilder;
use strategy::{Strategy, StrategyConfig};

#[tokio::main(flavor = "multi_thread", worker_threads = 220)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = tokio::task::spawn_blocking(|| -> rerun::RecordingStream {
        let open_browser = true;
        let rec = rerun::RecordingStreamBuilder::new("rerun_example_minimal_serve")
            .serve(
                "0.0.0.0",
                Default::default(),
                Default::default(),
                rerun::MemoryLimit::from_fraction_of_total(0.25),
                open_browser,
            )
            .expect("rerun error");
        rec
    })
    .await?;

    let df = import_parquet("data/kospi_tick.parquet")?;
    // let df = df.slice(0, 10);

    let mut simulation = SimulationBuilder::new(5, rec);

    simulation.market.add_ticks(df);
    for trader in simulation.traders.iter_mut() {
        trader.add_strategy(Strategy::new(StrategyConfig {
            name: "A".to_owned(),
            start_balance: 100_000_000_000.0f64,
            dca_ratio: 0.1,
            buy_begin: 9 * 3_600_000_000,
            buy_every: 2_000_000,
            sell_begin: 9 * 3_600_000_000 + 10 * 1_000_000,
            sell_every: 3_000_000,
        }));
        trader.report_nav_every(1_000_000);
    }

    simulation.run();
    tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;
    simulation.stop();

    Ok(())
}
