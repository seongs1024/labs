use crate::market::Tick;

pub trait Strategy {
	fn update_sec_codes(&mut self, sec_codes: Vec<String>);
	fn signal(&mut self, tick: Tick);
}

pub struct StrategyA;

impl Strategy for StrategyA {
	fn update_sec_codes(&mut self, sec_codes: Vec<String>) {
		
	}
	
	fn signal(&mut self, tick: Tick) {
		let Tick{ idx, time, code, price } = tick;
	}
}

impl StrategyA {
	pub fn new() -> Self {
		Self
	}
}
