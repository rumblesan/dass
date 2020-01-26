use std::cmp::Eq;
use std::fmt::Display;
use std::iter::Peekable;
use std::result::Result;
use std::vec::IntoIter;

use super::error::ParserError;
use super::tokens::TokenData;

pub struct DassParser<T>
where
    T: Clone + Display,
{
    tokens: Peekable<IntoIter<TokenData<T>>>,
}

impl<T> DassParser<T>
where
    T: Clone + Display + Eq,
{
    pub fn from_vec(tokens: Vec<TokenData<T>>) -> Self {
        DassParser {
            tokens: tokens.into_iter().peekable(),
        }
    }
    pub fn eof(&self) -> bool {
        self.tokens.len() == 0
    }
    pub fn la1(&mut self, tag: T) -> bool {
        match self.tokens.peek() {
            Some(t) => t.tag == tag,
            _ => false,
        }
    }
    pub fn match_token(&mut self, tag: T) -> Result<TokenData<T>, ParserError> {
        match self.tokens.next() {
            None => Err(ParserError::end_of_stream(tag)),
            Some(t) => {
                if t.tag != tag {
                    return Err(ParserError::unexpected_token(tag, t));
                } else {
                    return Ok(t);
                }
            }
        }
    }
    pub fn pop_token(&mut self) -> Result<TokenData<T>, ParserError> {
        self.tokens.next().ok_or(ParserError::new(
            String::from("To Pop token"),
            String::from("End of Stream"),
            None,
        ))
    }
}
