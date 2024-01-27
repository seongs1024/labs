use crate::{
    logger::{Event, Side},
    market::{Securities, Tick},
};
use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};
use std::{
    collections::HashSet,
    ops::Not,
    sync::{Arc, Mutex},
};

pub struct Strategy {
    name: String,
    sec_codes: HashSet<String>,
    prev_every_30min: i64,
    prev_every_10min: i64,
    bought: bool,
    sold: bool,
    rng: StdRng,
}

impl Strategy {
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

    pub async fn buy_signal(
        &mut self,
        tick: &Tick,
        trader_name: &str,
        sec_codes: &Securities,
    ) -> Option<Event> {
        let Tick {
            time, code, price, ..
        } = tick;

        let buy_start_on = time - 9 * 3_600_000_000i64;
        // let every_30min = buy_start_on % (30 * 60_000_000i64);
        let every_30min = buy_start_on % (100_000i64);

        if buy_start_on >= 0 && every_30min < self.prev_every_30min {
            self.bought = false;
        }
        self.prev_every_30min = every_30min;

        if buy_start_on >= 0 && self.bought.not() {
            // buy
            let candidate = {
                let sec_codes = sec_codes.read().await;
                (*sec_codes)
                    .keys()
                    .choose(&mut self.rng)
                    .unwrap()
                    .to_owned()
            };
            self.bought = true;
            return Some(Event::OpenOrder(
                Side::Buy,
                trader_name.to_owned(),
                *time,
                candidate,
                100.0,
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

        let sell_start_on = time - (14 * 3_600_000_000i64 + 30 * 60_000_000i64);
        let every_10min = sell_start_on % (10 * 60_000_000i64);

        if sell_start_on >= 0 && every_10min < self.prev_every_10min {
            // sell
        }
        self.prev_every_10min = every_10min;

        None
    }
}
