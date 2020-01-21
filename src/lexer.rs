use regex::Regex;
use std::fmt::{Debug, Display};

use super::position_tracker::PositionTracker;
use super::tokens::{TokenData, TokenMatcher};

pub type Token = TokenData<TokenTag>;

#[derive(Debug, Clone)]
pub enum TokenTag {
    End,
    WhiteSpace,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    SemiColon,
    Comma,
    Assignment,
    Operator,
    Float,
    Integer,
    Number,
    Identifier,
}

pub struct MeansLexer {
    matchers: Vec<TokenMatcher<TokenTag>>,
}

impl Display for TokenTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl MeansLexer {
    pub fn create() -> MeansLexer {
        MeansLexer {
            matchers: vec![
                TokenMatcher::whitespace_with_newlines(TokenTag::WhiteSpace),
                TokenMatcher::constant("(", TokenTag::OpenParen),
                TokenMatcher::constant(")", TokenTag::CloseParen),
                TokenMatcher::constant("{", TokenTag::OpenBrace),
                TokenMatcher::constant("}", TokenTag::CloseBrace),
                TokenMatcher::constant("[", TokenTag::OpenBracket),
                TokenMatcher::constant("]", TokenTag::CloseBracket),
                TokenMatcher::constant(";", TokenTag::SemiColon),
                TokenMatcher::constant("=", TokenTag::Assignment),
                TokenMatcher::constant(",", TokenTag::Comma),
                TokenMatcher::operators("+-*/%", TokenTag::Operator),
                TokenMatcher::float(TokenTag::Float),
                TokenMatcher::integer(TokenTag::Integer),
                TokenMatcher::regex(
                    Regex::new(r"^[[:alpha:]][[:alnum:]]*").unwrap(),
                    TokenTag::Identifier,
                ),
            ],
        }
    }
    pub fn tokenise(&self, input: &str) -> Vec<Token> {
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
