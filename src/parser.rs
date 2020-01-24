use std::cmp::Eq;
use std::fmt::Display;
use std::result::Result;

use super::error::ParserError;
use super::tokens::TokenData;

pub struct DassParser<T: Clone + Display + Eq> {
    tokens: Vec<TokenData<T>>,
}

impl<T: Clone + Display + Eq> DassParser<T> {
    pub fn new(mut tokens: Vec<TokenData<T>>) -> DassParser<T> {
        // reverse tokens so we can use `last` and `pop` methods
        tokens.reverse();
        DassParser { tokens }
    }
    pub fn eof(&self) -> bool {
        self.tokens.len() == 0
    }
    pub fn la1(&self, tag: T) -> bool {
        match self.tokens.last() {
            Some(t) => t.tag == tag,
            _ => false,
        }
    }
    pub fn match_token(&mut self, tag: T) -> Result<TokenData<T>, ParserError> {
        if self.eof() {
            return Err(ParserError::end_of_stream(tag));
        }
        let t = self.tokens.pop().unwrap();
        if t.tag != tag {
            return Err(ParserError::unexpected_token(tag, t));
        }
        Ok(t)
    }
    pub fn pop_token(&mut self) -> Result<TokenData<T>, ParserError> {
        match self.tokens.pop() {
            Some(t) => Ok(t),
            None => Err(ParserError::new(
                String::from("To Pop token"),
                String::from("End of Stream"),
                None,
            )),
        }
    }
}
