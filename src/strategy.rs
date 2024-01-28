use crate::{
    logger::{Event, Side},
    market::{Securities, Tick},
};
use std::{
    ops::{Div, Not, Rem},
    sync::{Arc, Mutex},
};

pub struct StrategyConfig {
    pub name: String,
    pub start_balance: f64,
    pub dca_ratio: f64,
    pub buy_begin: i64,
    pub buy_every: i64,
    pub sell_begin: i64,
    pub sell_every: i64,
}

pub struct Strategy {
    prev_buy_every: i64,
    prev_sell_every: i64,
    bought: bool,
    sold: bool,
    pub config: StrategyConfig,
}

impl Strategy {
    pub fn new(config: StrategyConfig) -> Self {
        Self {
            prev_buy_every: 0,
            prev_sell_every: 0,
            bought: false,
            sold: false,
            config,
        }
    }

    pub async fn buy_signal(
        &mut self,
        tick: &Tick,
        trader_name: &str,
        cash: f64,
        sec_codes: &Securities,
    ) -> Option<Event> {
        let Tick {
            time, code, price, ..
        } = tick;

        let buy_start_on = time - self.config.buy_begin;
        let buy_every = buy_start_on % self.config.buy_every;

        if buy_start_on >= 0 && buy_every < self.prev_buy_every {
            self.bought = false;
        }
        self.prev_buy_every = buy_every;

        let available = cash.min((self.config.start_balance * self.config.dca_ratio).floor());
        if buy_start_on >= 0 && self.bought.not() && available > 0.0 {
            // buy
            let (candidate, price) = {
                let sec_codes = sec_codes.read().await;
                let candidates = (*sec_codes)
                    .iter()
                    .filter(|(_, price)| price <= &&cash)
                    .collect::<Vec<_>>();
                if candidates.len() == 0 {
                    return None;
                }
                let (candidate, price) =
                    candidates[(rand::random::<usize>().rem(candidates.len()) as usize)];
                (candidate.to_owned(), *price)
            };
            let quantity = available.div(price).floor() as i64;
            self.bought = true;
            return Some(Event::OpenOrder(
                Side::Buy,
                trader_name.to_owned(),
                self.config.name.to_owned(),
                *time,
                candidate,
                quantity,
            ));
        }

        None
    }

    pub async fn sell_signal(
        &mut self,
        tick: &Tick,
        trader_name: &str,
        sec_codes: &Securities,
    ) -> Option<Event> {
        let Tick {
            time, code, price, ..
        } = tick;

        // let sell_start_on = time - (14 * 3_600_000_000i64 + 30 * 60_000_000i64);
        // let every_10min = sell_start_on % (10 * 60_000_000i64);

        // if sell_start_on >= 0 && every_10min < self.prev_every_10min {
        //     // sell
        // }
        // self.prev_every_10min = every_10min;

        None
    }
}
