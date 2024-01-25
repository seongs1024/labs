use crate::market::Tick;
use std::collections::HashSet;

pub trait Strategy {
    fn update_sec_codes(&mut self, sec_codes: HashSet<String>);
    fn signal(&mut self, tick: Tick);
}

pub struct StrategyA {
    sec_codes: HashSet<String>,
}

impl Strategy for StrategyA {
    fn update_sec_codes(&mut self, sec_codes: HashSet<String>) {
        self.sec_codes = sec_codes;
    }

    fn signal(&mut self, tick: Tick) {
        let Tick {
            idx,
            time,
            code,
            price,
        } = tick;
        self.sec_codes.insert(code.clone());
    }
}

impl StrategyA {
    pub fn new() -> Self {
        Self {
            sec_codes: HashSet::new(),
        }
    }
}
