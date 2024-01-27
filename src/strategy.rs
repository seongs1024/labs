use crate::{
    logger::{Event, Side},
    market::Tick,
};
use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};
use std::{
    collections::HashSet,
    ops::Not,
    sync::{Arc, Mutex},
};

pub trait Strategy {
    fn update_sec_codes(&mut self, sec_codes: HashSet<String>);
    fn buy_signal(&mut self, tick: &Tick, trader_name: &str) -> Option<Event>;
    fn sell_signal(&mut self, tick: &Tick, trader_name: &str) -> Option<Event>;
}

pub struct StrategyA {
    name: String,
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

    fn buy_signal(&mut self, tick: &Tick, trader_name: &str) -> Option<Event> {
        let Tick {
            time, code, price, ..
        } = tick;
        self.sec_codes.insert(code.clone());

        let buy_start_on = time - 9 * 3_600_000_000i64;
        // let every_30min = buy_start_on % (30 * 60_000_000i64);
        let every_30min = buy_start_on % (100_000i64);

        if buy_start_on >= 0 && every_30min < self.prev_every_30min {
            self.bought = false;
        }
        self.prev_every_30min = every_30min;

        if buy_start_on >= 0 && self.bought.not() {
            // buy
            let candidate = self.sec_codes.iter().choose(&mut self.rng).unwrap();
            self.bought = true;
            return Some(Event::OpenOrder(
                Side::Buy,
                trader_name.to_owned(),
                *time,
                candidate.to_owned(),
                100.0,
            ));
        }

        None
    }

    fn sell_signal(&mut self, tick: &Tick, trader_name: &str) -> Option<Event> {
        let Tick {
            time, code, price, ..
        } = tick;
        self.sec_codes.insert(code.clone());

        let sell_start_on = time - (14 * 3_600_000_000i64 + 30 * 60_000_000i64);
        let every_10min = sell_start_on % (10 * 60_000_000i64);

        if sell_start_on >= 0 && every_10min < self.prev_every_10min {
            // sell
        }
        self.prev_every_10min = every_10min;

        None
    }
}

impl StrategyA {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            sec_codes: HashSet::new(),
            prev_every_30min: 0,
            prev_every_10min: 0,
            bought: false,
            sold: false,
            rng: StdRng::seed_from_u64(rand::random()),
        }
    }
}
