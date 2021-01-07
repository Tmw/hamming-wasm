#[derive(Clone, PartialEq)]
pub struct Bit {
    pub is_high: bool,
    pub is_flipped: bool,
    pub index: usize,
    pub index_in_block: u8,
}

impl Bit {
    pub fn is_parity(&self) -> bool {
        self.index_in_block.is_power_of_two()
    }

    pub fn flip(&mut self) {
        self.is_flipped = !self.is_flipped;
        self.is_high = !self.is_high;
    }
}
