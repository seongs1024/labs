use crate::{
    market::{Itcp, Market},
    trader::Trader,
};
use polars::prelude::*;
use tokio::sync::broadcast;

// Market and Strategies Builder
pub struct MtBuilder;

impl MtBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(df: DataFrame, strategies: usize) -> (Market<Itcp>, Vec<Trader<Itcp>>) {
        let (tx, rx) = broadcast::channel(1_000);

        if strategies < 2 {
            todo!();
        }
        let strategies: Vec<_> = (0..(strategies - 1))
            .map(|_| tx.subscribe())
            .chain(std::iter::once(rx))
            .enumerate()
            .map(|(i, rx)| Trader::new(format!("{}", i), rx))
            .collect();
        let market = Market::new(df, tx);
        (market, strategies)
    }
}
