use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct StreamPosition {
    pub line: u64,
    pub character: u64,
}

impl Display for StreamPosition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "char {} on line {}", self.character, self.line)
    }
}

pub struct PositionTracker {
    pub line: u64,
    pub character: u64,
}

impl PositionTracker {
    pub fn new() -> PositionTracker {
        PositionTracker {
            line: 1,
            character: 1,
        }
    }
    pub fn consume(&mut self, input: &str) {
        for c in input.chars() {
            if c == '\n' {
                self.line += 1;
                self.character = 1;
            } else {
                self.character += 1;
            }
        }
    }
    pub fn position(&self) -> StreamPosition {
        StreamPosition {
            line: self.line,
            character: self.character,
        }
    }
}
