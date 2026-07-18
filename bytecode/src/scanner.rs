use std::iter::Peekable;
use std::str::CharIndices;

use crate::token::Token;

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { source }
    }

    pub fn lexemes(&self) -> Lexemes<'_> {
        Lexemes {
            source: &self.source,
            chars: self.source.char_indices().peekable(),
            line: 0,
            done: false,
        }
    }
}

pub struct Lexemes<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    line: u32,
    done: bool,
}

impl<'a> Lexemes<'a> {
    fn consume_if(&mut self, expected: char) -> bool {
        self.chars.next_if(|&(_, c)| c == expected).is_some()
    }

    fn skip_whitespace(&mut self) -> bool {
        let mut skipped = false;
        while let Some((_, c)) = self.chars.next_if(|&(_, c)| c.is_whitespace()) {
            if c == '\n' {
                self.line += 1;
            }
            skipped = true;
        }
        skipped
    }

    fn skip_comment(&mut self) -> bool {
        match self.chars.peek() {
            Some(&(i, '/')) if self.source[i..].starts_with("//") => {
                while self.chars.next_if(|&(_, c)| c != '\n').is_some() {}
                true
            }
            _ => false,
        }
    }

    fn skip_ignored(&mut self) {
        while self.skip_whitespace() || self.skip_comment() {}
    }

    fn string(&mut self, open: usize) -> Token<'a> {
        let start = open + 1;
        while let Some((_, c)) = self.chars.next_if(|&(_, c)| c != '"') {
            if c == '\n' {
                self.line += 1;
            }
        }

        match self.chars.next() {
            Some((end, '"')) => Token::String(&self.source[start..end]),
            _ => panic!("unterminated string on line {}", self.line + 1),
        }
    }
}

impl<'a> Iterator for Lexemes<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_ignored();

        let (i, ch) = match self.chars.next() {
            None => {
                if !self.done {
                    self.done = true;
                    return Some(Token::EOF);
                } else {
                    return None;
                }
            }
            Some(c) => c,
        };

        let token = match ch {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '-' => Token::Minus,
            '+' => Token::Plus,
            '/' => Token::Slash,
            '*' => Token::Star,
            '"' => self.string(i),
            '!' => {
                if self.consume_if('=') {
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }
            '=' => {
                if self.consume_if('=') {
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '<' => {
                if self.consume_if('=') {
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if self.consume_if('=') {
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            _ => todo!(),
        };

        Some(token)
    }
}
