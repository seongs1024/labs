use crate::{
    logger::Logger,
    market::{Market, Securities},
    trader::Trader,
};
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
    pub fn new(strategies: usize, rerun: rerun::RecordingStream) -> Simulation {
        let (tx, rx) = broadcast::channel(100_000);
        let (log_tx, log_rx) = mpsc::channel(100_000);

        if strategies < 2 {
            todo!();
        }

        let sec_codes: Securities = Default::default();
        let traders: Vec<_> = (0..(strategies - 1))
            .map(|_| {
                (
                    tx.subscribe(),
                    log_tx.clone(),
                    rerun.clone(),
                    sec_codes.clone(),
                )
            })
            .collect();
        let traders: Vec<_> = traders
            .into_iter()
            .chain(std::iter::once((
                rx,
                log_tx,
                rerun.clone(),
                sec_codes.clone(),
            )))
            .enumerate()
            .map(|(i, (rx, log_tx, rerun, sec_codes))| {
                Trader::new(format!("{}", i), rx, log_tx, rerun, sec_codes)
            })
            .collect();
        let market = Market::new(tx, rerun.clone(), sec_codes);
        let logger = Logger::new(log_rx, rerun.clone());

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
        self.simulation_handlers.push(self.logger.recv());

        for trader in self.traders.iter_mut() {
            trader.recv();
        }

        self.simulation_handlers.push(self.market.send());
    }

    pub fn stop(&mut self) {
        for handle in &self.simulation_handlers {
            handle.abort();
        }
        self.simulation_handlers.clear();
    }
}
