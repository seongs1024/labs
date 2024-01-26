use crate::{logger::Logger, market::Market, strategy::StrategyA, trader::Trader};
use polars::prelude::*;
use std::ops::Not;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinHandle,
};

pub struct SimulationBuilder;

pub struct Simulation {
    simulation_handlers: Vec<JoinHandle<()>>,
    pub market: Market,
    pub traders: Vec<Trader>,
    pub logger: Logger,
}

impl SimulationBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(strategies: usize) -> Simulation {
        let (tx, rx) = broadcast::channel(100_000);
        let (log_tx, log_rx) = mpsc::channel(100_000);

        if strategies < 2 {
            todo!();
        }
        let traders: Vec<_> = (0..(strategies - 1))
            .map(|_| (tx.subscribe(), log_tx.clone()))
            .collect();
        let traders: Vec<_> = traders
            .into_iter()
            .chain(std::iter::once((rx, log_tx)))
            .enumerate()
            .map(|(i, (rx, log_tx))| Trader::new(format!("{}", i), rx, log_tx))
            .collect();
        let market = Market::new(tx);
        let logger = Logger::new(log_rx);

        Simulation {
            simulation_handlers: Vec::new(),
            market,
            traders,
            logger,
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

        self.simulation_handlers.push(self.market.send());
        self.simulation_handlers.push(self.logger.recv());
    }

    pub fn stop(&mut self) {
        for handle in &self.simulation_handlers {
            handle.abort();
        }
        self.simulation_handlers.clear();
    }
}
