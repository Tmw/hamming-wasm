use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum RenderingMode {
    Sequential,
    Blocks,
}

impl fmt::Display for RenderingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

use RenderingMode::{Blocks, Sequential};
impl RenderingMode {
    pub fn toggle(&mut self) {
        *self = self.opposite()
    }

    pub fn opposite(&self) -> Self {
        match self {
            Sequential => Blocks,
            Blocks => Sequential,
        }
    }
}
