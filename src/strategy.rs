use crate::market::Tick;
use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};
use std::{
    collections::HashSet,
    ops::Not,
    sync::{Arc, Mutex},
};

pub trait Strategy {
    fn update_sec_codes(&mut self, sec_codes: HashSet<String>);
    fn signal(&mut self, tick: Tick, trader_name: &str);
}

pub struct StrategyA {
    sec_codes: HashSet<String>,
    prev_every_30min: i64,
    prev_every_10min: i64,
    bought: bool,
    sold: bool,
    rng: StdRng,
}

impl Strategy for StrategyA {
    fn update_sec_codes(&mut self, sec_codes: HashSet<String>) {
        self.sec_codes = sec_codes;
    }

    fn signal(&mut self, tick: Tick, trader_name: &str) {
        let Tick {
            time, code, price, ..
        } = tick;
        self.sec_codes.insert(code.clone());

        let buy_start_on = time - 9 * 3_600_000_000i64;
        let sell_start_on = time - (14 * 3_600_000_000i64 + 30 * 60_000_000i64);
        let every_30min = buy_start_on % (30 * 60_000_000i64);
        let every_10min = sell_start_on % (10 * 60_000_000i64);

        if buy_start_on >= 0 && every_30min < self.prev_every_30min {
            self.bought = false;
        }
        if buy_start_on >= 0 && self.bought.not() {
            // buy
            // println!("{}: bought {}!", trader_name, self.sec_codes.iter().choose(&mut self.rng).unwrap());
            println!("{}: bought {}!", trader_name, code);
            self.bought = true;
        }
        self.prev_every_30min = every_30min;

        if sell_start_on >= 0 && every_10min < self.prev_every_10min {
            // sell
        }
        self.prev_every_10min = every_10min;
    }
}

impl StrategyA {
    pub fn new() -> Self {
        Self {
            sec_codes: HashSet::new(),
            prev_every_30min: 0,
            prev_every_10min: 0,
            bought: false,
            sold: false,
            rng: StdRng::seed_from_u64(0),
        }
    }
}
