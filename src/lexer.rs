use std::fmt::Display;

use super::error::ParserError;
use super::position_tracker::PositionTracker;
use super::tokens::{TokenData, TokenMatcher};

pub struct DassLexerBuilder<T>
where
    T: Clone + Display,
{
    matchers: Vec<TokenMatcher<T>>,
}

impl<T> DassLexerBuilder<T>
where
    T: Clone + Display,
{
    pub fn new(matchers: Vec<TokenMatcher<T>>) -> Self {
        DassLexerBuilder { matchers }
    }
    pub fn build<'a>(&self, source: &'a str) -> DassLexer<'a, T> {
        DassLexer {
            matchers: self.matchers.clone(),
            tracker: PositionTracker::new(),
            source,
        }
    }
}

#[derive(Clone)]
pub struct DassLexer<'a, T>
where
    T: Clone + Display,
{
    matchers: Vec<TokenMatcher<T>>,
    tracker: PositionTracker,
    source: &'a str,
}

impl<'a, T> Iterator for DassLexer<'a, T>
where
    T: Clone + Display,
{
    type Item = Result<TokenData<T>, ParserError>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.source.len() < 1 {
                return None;
            }
            let mut matched = false;

            for matcher in &self.matchers {
                if matcher.regex.is_match(&self.source) {
                    matched = true;
                    let m = matcher.regex.find(&self.source).unwrap();
                    let s = m.as_str();
                    self.source = &self.source[s.len()..];
                    if !matcher.skip {
                        let res = Some(Ok(matcher.parse(
                            s,
                            self.tracker.line,
                            self.tracker.character,
                        )));
                        self.tracker.consume(s);
                        return res;
                    } else {
                        self.tracker.consume(s);
                    }
                }
            }

            if !matched {
                let c = &self.source[..1];
                let err = Some(Err(ParserError::new(
                    String::from("lexing match"),
                    c.to_owned(),
                    Some(self.tracker.position()),
                )));
                self.source = &self.source[1..];
                self.tracker.consume(c);
                return err;
            }
        }
    }
}
