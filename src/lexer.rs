use std::fmt::Display;

use super::position_tracker::PositionTracker;
use super::tokens::{TokenData, TokenMatcher};

pub struct DassLexer<T: Clone + Display> {
    matchers: Vec<TokenMatcher<T>>,
}

impl<T: Clone + Display> DassLexer<T> {
    pub fn create(matchers: Vec<TokenMatcher<T>>) -> DassLexer<T> {
        DassLexer { matchers }
    }
    pub fn tokenise(&self, input: &str) -> Vec<TokenData<T>> {
        let mut tracker = PositionTracker::new();
        let mut source = input;
        let mut tokens = Vec::new();
        while source.len() > 0 {
            let mut matched = false;
            for matcher in &self.matchers {
                if matcher.regex.is_match(&source) {
                    matched = true;
                    let m = matcher.regex.find(&source).unwrap();
                    let s = m.as_str();
                    if !matcher.skip {
                        let t = matcher.parse(s, tracker.line, tracker.character);
                        tokens.push(t);
                    }
                    tracker.consume(s);
                    source = &source[s.len()..];
                }
            }
            if !matched {
                tracker.consume(&source[..1]);
                source = &source[1..];
                println!("Matching Error!");
            }
        }
        return tokens;
    }
}
