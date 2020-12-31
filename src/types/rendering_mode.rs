use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum RenderingMode {
    Sequential,
    Blocks
}

impl fmt::Display for RenderingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

use RenderingMode::{Sequential, Blocks};
impl RenderingMode {
    pub fn toggle(&mut self){
        match self {
            Sequential => *self = Blocks,
            Blocks => *self = Sequential,
        };
    }
}
