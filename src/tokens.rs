use regex::Regex;
use std::fmt::Display;

#[derive(Debug)]
pub struct TokenData<T: Clone + Display> {
    pub tag: T,
    pub contents: String,
    pub line: u64,
    pub character: u64,
}

impl<T: Clone + Display> std::fmt::Display for TokenData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({} at l{}.{} :: {})",
            self.tag, self.line, self.character, self.contents
        )
    }
}

pub struct TokenMatcher<T: Clone + Display> {
    pub regex: Regex,
    pub tag: T,
    pub skip: bool,
}

impl<T: Clone + Display> TokenMatcher<T> {
    pub fn new(regex: Regex, tag: T, skip: bool) -> TokenMatcher<T> {
        TokenMatcher { regex, tag, skip }
    }
    pub fn parse(&self, contents: &str, line: u64, character: u64) -> TokenData<T> {
        TokenData {
            tag: self.tag.clone(),
            contents: contents.to_owned(),
            line,
            character,
        }
    }
    pub fn regex(regex: Regex, tag: T) -> TokenMatcher<T> {
        TokenMatcher::new(regex, tag, false)
    }
    pub fn whitespace(tag: T) -> TokenMatcher<T> {
        let regex = Regex::new(r"^\s\t").unwrap();
        TokenMatcher::new(regex, tag, true)
    }
    pub fn whitespace_with_newlines(tag: T) -> TokenMatcher<T> {
        let regex = Regex::new(r"^[[:space:]]+").unwrap();
        TokenMatcher::new(regex, tag, true)
    }
    pub fn constant(symbol: &str, tag: T) -> TokenMatcher<T> {
        let pattern = format!("^{}", regex::escape(symbol));
        let regex = Regex::new(&pattern).unwrap();
        TokenMatcher::new(regex, tag, false)
    }
    pub fn operators(symbols: &str, tag: T) -> TokenMatcher<T> {
        let pattern = format!("^[{}]+", regex::escape(symbols));
        let regex = Regex::new(&pattern).unwrap();
        TokenMatcher::new(regex, tag, false)
    }
    pub fn float(tag: T) -> TokenMatcher<T> {
        let regex = Regex::new(r"^[0-9]+\.[0-9]+").unwrap();
        TokenMatcher::new(regex, tag, false)
    }
    pub fn number(tag: T) -> TokenMatcher<T> {
        let regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        TokenMatcher::new(regex, tag, false)
    }
    pub fn integer(tag: T) -> TokenMatcher<T> {
        let regex = Regex::new(r"^[0-9]+").unwrap();
        TokenMatcher::new(regex, tag, false)
    }
}
