use std::cmp::Eq;
use std::fmt::Display;
use std::iter::Peekable;
use std::result::Result;
use std::vec::IntoIter;

use super::error::ParserError;
use super::lexer::DassLexer;
use super::tokens::TokenData;

pub trait DassParser<T>
where
    T: Clone + Display,
{
    fn eof(&mut self) -> bool;

    fn la1(&mut self, tag: &T) -> bool;

    fn match_token(&mut self, tag: &T) -> Result<TokenData<T>, ParserError>;

    fn pop_token(&mut self) -> Result<TokenData<T>, ParserError>;

    fn pop_until(&mut self, tag: &T) -> Result<TokenData<T>, ParserError> {
        while !self.la1(tag) {
            let t = self.pop_token();
            if t.is_err() {
                return t;
            }
        }
        self.pop_token()
    }
}

pub struct DassVecStreamTokenParser<T>
where
    T: Clone + Display,
{
    tokens: Peekable<IntoIter<TokenData<T>>>,
}

impl<T> DassVecStreamTokenParser<T>
where
    T: Clone + Display,
{
    pub fn new(tokens: Vec<TokenData<T>>) -> Self {
        DassVecStreamTokenParser {
            tokens: tokens.into_iter().peekable(),
        }
    }
}

impl<T> DassParser<T> for DassVecStreamTokenParser<T>
where
    T: Clone + Display + Eq,
{
    fn eof(&mut self) -> bool {
        self.tokens.peek().is_none()
    }
    fn la1(&mut self, tag: &T) -> bool {
        self.tokens.peek().map_or(false, |t| t.tag == *tag)
    }
    fn match_token(&mut self, tag: &T) -> Result<TokenData<T>, ParserError> {
        self.tokens
            .next()
            .map_or(Err(ParserError::end_of_stream(&tag)), |t| {
                if t.tag != *tag {
                    Err(ParserError::unexpected_token(tag, t))
                } else {
                    Ok(t)
                }
            })
    }
    fn pop_token(&mut self) -> Result<TokenData<T>, ParserError> {
        self.tokens.next().ok_or(ParserError::new(
            String::from("To Pop token"),
            String::from("End of Stream"),
            None,
        ))
    }
}

pub struct DassTokenParser<'a, T>
where
    T: Clone + Display,
{
    lexer: Peekable<DassLexer<'a, T>>,
}

impl<'a, T> DassTokenParser<'a, T>
where
    T: Clone + Display,
{
    pub fn new(lexer: DassLexer<'a, T>) -> Self {
        DassTokenParser {
            lexer: lexer.peekable(),
        }
    }
}

impl<'a, T> DassParser<T> for DassTokenParser<'a, T>
where
    T: Clone + Display + Eq,
{
    fn eof(&mut self) -> bool {
        self.lexer.peek().is_none()
    }
    fn la1(&mut self, tag: &T) -> bool {
        self.lexer.peek().map_or(false, |res| match res {
            Err(_) => false,
            Ok(t) => t.tag == *tag,
        })
    }
    fn match_token(&mut self, tag: &T) -> Result<TokenData<T>, ParserError> {
        self.lexer
            .next()
            .map_or(Err(ParserError::end_of_stream(tag)), |res| {
                res.and_then(|t| {
                    if t.tag != *tag {
                        Err(ParserError::unexpected_token(tag, t))
                    } else {
                        Ok(t)
                    }
                })
            })
    }
    fn pop_token(&mut self) -> Result<TokenData<T>, ParserError> {
        match self.lexer.next() {
            Some(t) => t,
            None => Err(ParserError::new(
                String::from("To Pop token"),
                String::from("End of Stream"),
                None,
            )),
        }
    }
}
