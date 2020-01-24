use std::fmt::Display;

use super::error::ParserError;
use super::position_tracker::PositionTracker;
use super::tokens::{TokenData, TokenMatcher};

pub struct DassLexer<T: Clone + Display> {
    matchers: Vec<TokenMatcher<T>>,
}

pub struct LexerResults<T: Clone + Display> {
    tokens: Vec<TokenData<T>>,
    errors: Vec<ParserError>,
}

impl<T: Clone + Display> DassLexer<T> {
    pub fn create(matchers: Vec<TokenMatcher<T>>) -> DassLexer<T> {
        DassLexer { matchers }
    }
    pub fn tokenise(&self, input: &str) -> LexerResults<T> {
        let mut tracker = PositionTracker::new();
        let mut source = input;
        let mut result = LexerResults {
            tokens: Vec::new(),
            errors: Vec::new(),
        };
        while source.len() > 0 {
            let mut matched = false;
            for matcher in &self.matchers {
                if matcher.regex.is_match(&source) {
                    matched = true;
                    let m = matcher.regex.find(&source).unwrap();
                    let s = m.as_str();
                    if !matcher.skip {
                        let t = matcher.parse(s, tracker.line, tracker.character);
                        result.tokens.push(t);
                    }
                    tracker.consume(s);
                    source = &source[s.len()..];
                }
            }
            if !matched {
                tracker.consume(&source[..1]);
                source = &source[1..];
                result.errors.push(ParserError::new(
                    String::from("lexing match"),
                    String::from(source),
                    Some(tracker.position()),
                ));
                println!("Matching Error!");
            }
        }
        return result;
    }
}
