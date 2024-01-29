use crate::{
    logger::{Event, Side},
    market::{Securities, Tick},
};
use std::{
    collections::HashMap,
    ops::{Div, Not, Rem},
    sync::{Arc, Mutex},
};

#[derive(Clone)]
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
        stocks_held: &HashMap<String, i64>,
    ) -> Option<Event> {
        let Tick {
            time, code, price, ..
        } = tick;

        let sell_start_on = time - self.config.sell_begin;
        let sell_every = sell_start_on % self.config.sell_every;

        if sell_start_on >= 0 && sell_every < self.prev_sell_every {
            self.sold = false;
        }
        self.prev_sell_every = sell_every;

        if sell_start_on >= 0 && self.sold.not() {
            // sell
            let (candidate, quantity) = match stocks_held.iter().next() {
                    Some(candidate) => candidate,
                    None => return None,
                };
            self.sold = true;
            return Some(Event::OpenOrder(
                Side::Sell,
                trader_name.to_owned(),
                self.config.name.to_owned(),
                *time,
                candidate.to_owned(),
                *quantity,
            ));
        }

        None
    }
}
