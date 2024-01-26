mod data_loader;
mod market;
mod simulation_builder;
mod strategy;
mod trader;

use data_loader::import_parquet;
use simulation_builder::SimulationBuilder;
use strategy::StrategyA;

#[tokio::main(flavor = "multi_thread", worker_threads = 220)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = import_parquet("data/kospi_tick.parquet")?;
    // let df = df.slice(0, 10);

    let mut simulation = SimulationBuilder::new(200);

    simulation.market.add_ticks(df);
    for trader in simulation.traders.iter_mut() {
        trader.add_strategy(StrategyA::new());
    }

    simulation.run();
    tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;
    simulation.stop();
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    Ok(())
}
