//! A lot of the parser/lexer code is a modified version of the rustc parser

use std::fmt::{Display, Formatter, Result};
use std::ops::Range;

use regex::Regex;

use self::Tokens::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn len(self) -> usize {
        self.end - self.start
    }

    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }

    pub fn as_range(self) -> Range<usize> {
        self.start..self.end
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Token {
    pub kind: Tokens,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub kind: Tokens,
    pub re: Regex,
}

pub struct Lexer {
    rules: Vec<Rule>,
    pub consumed_len: usize,
    pub input_str: String,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum Tokens {
    Dot,
    Comma,
    OpenSquare,
    CloseSquare,
    Plus,
    Minus,
    ShiftR,
    ShiftL,
    Comment,
    Newline,
    Unknown,
}

impl Lexer {
    pub fn build(rules: Vec<Rule>) -> Self {
        Lexer {
            rules,
            consumed_len: 0,
            input_str: String::new(),
        }
    }

    pub fn tokenize(&mut self, input: &str) -> Vec<Token> {
        let mut val = Vec::new();
        self.input_str = input.to_string();
        let mut rem = input; // Remaining input
        while !rem.is_empty() {
            let token = self.next_token(rem);
            let len = token.span.len() as usize;

            val.push(token);
            rem = &rem[len..];
        }
        val
    }

    pub fn next_token(&mut self, input: &str) -> Token {
        self.token_valid(input)
            .unwrap_or_else(|| self.token_invalid(input))
    }

    fn token_valid(&mut self, input: &str) -> Option<Token> {
        let longest = self
            .rules
            .iter()
            .rev()
            .filter_map(|rule| {
                let mch = rule.re.find(input)?;
                Some((mch.end(), rule))
            })
            .max_by_key(|&(len, _)| len)?;

        let (len, rule) = longest;
        let prev_len = self.consumed_len;
        self.consumed_len += len;
        let token_span = Span::new(prev_len, self.consumed_len);
        let kind_cl = rule.kind.clone();
        assert!(
            len > 0,
            "Bad token\nkind: {:?}\nregex: {:?}\ninput {:?}",
            rule.kind,
            rule.re,
            input
        );
        Some(Token {
            kind: kind_cl,
            span: token_span,
        })
    }

    fn token_invalid(&mut self, input: &str) -> Token {
        let mut len = 0;
        let prev_len = self.consumed_len;
        for c in input.chars() {
            len += c.len_utf8();
            if self.token_valid(&input[len..]).is_some() {
                break;
            }
        }
        self.consumed_len += len;
        Token {
            kind: Unknown,
            span: Span::new(prev_len, self.consumed_len),
        }
    }
}