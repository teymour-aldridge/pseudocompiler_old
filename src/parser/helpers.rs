pub struct NumberState {
    exponent: bool,
    decimal: bool,
}

impl NumberState {
    pub fn new() -> Self {
        Self {
            exponent: false,
            decimal: false,
        }
    }
    pub fn set_exp(&mut self, val: bool) {
        self.exponent = val;
    }
    pub fn set_dec(&mut self, val: bool) {
        self.decimal = val;
    }
}