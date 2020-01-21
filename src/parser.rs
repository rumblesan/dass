use std::cmp::Eq;
use std::fmt::Display;
use std::result::Result;

use super::tokens::TokenData;

pub enum ParserError<T: Clone + Display + Eq> {
    UnexpectedEndOfFile,
    UnexpectedEndOfStream,
    UnexpectedToken { expected: T, found: TokenData<T> },
}

pub struct DassParser<T: Clone + Display + Eq> {
    tokens: Vec<TokenData<T>>,
}

impl<T: Clone + Display + Eq> DassParser<T> {
    pub fn new(tokens: Vec<TokenData<T>>) -> DassParser<T> {
        DassParser { tokens }
    }
    pub fn eof(&self) -> bool {
        self.tokens.len() == 0
    }
    pub fn la1(&self, tag: T) -> bool {
        if self.eof() {
            return false;
        }
        self.tokens[0].tag == tag
    }
    pub fn match_token(&mut self, tag: T) -> Result<TokenData<T>, ParserError<T>> {
        if self.eof() {
            return Err(ParserError::UnexpectedEndOfFile);
        }
        let t = self.tokens.pop().unwrap();
        if t.tag != tag {
            return Err(ParserError::UnexpectedToken {
                expected: tag,
                found: t,
            });
        }
        Ok(t)
    }
    pub fn pop_token(&mut self) -> Option<TokenData<T>> {
        self.tokens.pop()
    }
}
