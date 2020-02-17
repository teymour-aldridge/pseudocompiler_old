#[derive(Debug)]
pub struct NumberState {
	pub exponent: bool,
	pub decimal: bool,
}

impl NumberState {
	pub fn new() -> Self {
		Self { exponent: false, decimal: false }
	}
	pub fn set_exp(&mut self, val: bool) {
		self.exponent = val;
		if val {
			self.decimal = false;
		}
	}
	pub fn set_dec(&mut self, val: bool) {
		self.decimal = val;
	}
}
