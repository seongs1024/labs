mod data_loader;
mod logger;
mod market;
mod simulation_builder;
mod strategy;
mod trader;

use data_loader::import_parquet;
use simulation_builder::SimulationBuilder;
use strategy::Strategy;

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

    let mut simulation = SimulationBuilder::new(200, rec);

    simulation.market.add_ticks(df);
    for trader in simulation.traders.iter_mut() {
        trader.add_strategy(Strategy::new("Strategy A"));
    }

    simulation.run();
    tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;
    simulation.stop();
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    Ok(())
}
