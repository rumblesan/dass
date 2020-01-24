use std::fmt::{Display, Formatter, Result};

use super::position_tracker::StreamPosition;
use super::tokens::TokenData;

#[derive(Debug)]
pub struct ParserError {
    expected: String,
    found: String,
    position: Option<StreamPosition>,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.position {
            Some(pos) => write!(
                f,
                "Unexpected {} at {}. Expected {}",
                self.found, pos, self.expected
            ),
            None => write!(f, "Unexpected {}. Expected {}", self.found, self.expected),
        }
    }
}

impl ParserError {
    pub fn new(expected: String, found: String, position: Option<StreamPosition>) -> Self {
        ParserError {
            expected,
            found,
            position,
        }
    }
    pub fn end_of_stream<T: Display>(expected: T) -> Self {
        ParserError {
            expected: format!("{}", expected),
            found: String::from("End of Stream"),
            position: None,
        }
    }
    pub fn unexpected_token<T: Display + Clone>(expected: T, found: TokenData<T>) -> Self {
        ParserError {
            expected: format!("{}", expected),
            found: String::from("End of Stream"),
            position: Some(found.position()),
        }
    }
}
