use crate::{market::Market, strategy::StrategyA, trader::Trader};
use polars::prelude::*;
use std::ops::Not;
use tokio::{sync::broadcast, task::JoinHandle};

pub struct SimulationBuilder;

pub struct Simulation {
    simulation_handlers: Vec<JoinHandle<()>>,
    pub market: Market,
    pub traders: Vec<Trader>,
}

impl SimulationBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(strategies: usize) -> Simulation {
        let (tx, rx) = broadcast::channel(100_000);

        if strategies < 2 {
            todo!();
        }
        let traders: Vec<_> = (0..(strategies - 1))
            .map(|_| tx.subscribe())
            .chain(std::iter::once(rx))
            .enumerate()
            .map(|(i, rx)| Trader::new(format!("{}", i), rx))
            .collect();
        let market = Market::new(tx);

        Simulation {
            simulation_handlers: Vec::new(),
            market,
            traders,
        }
    }
}

impl Simulation {
    pub fn run(&mut self) {
        if self.market.is_ok().not() || self.traders.iter().any(|trader| trader.is_ok().not()) {
            return;
        }
        for trader in self.traders.iter_mut() {
            trader.recv();
        }

        let handle = self.market.send();

        self.simulation_handlers.push(handle);
    }

    pub fn stop(&mut self) {
        for handle in &self.simulation_handlers {
            handle.abort();
        }
        self.simulation_handlers.clear();
    }
}
