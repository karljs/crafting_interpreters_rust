use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;

pub struct Scanner {
    line: u32,
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { line: 0, source }
    }

    pub fn lexemes(&self) -> Lexemes<'_> {
        Lexemes {
            chars: self.source.chars().peekable(),
            done: false,
        }
    }
}

pub struct Lexemes<'a> {
    chars: Peekable<Chars<'a>>,
    done: bool,
}

impl<'a> Iterator for Lexemes<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while matches!(self.chars.peek(), Some(c) if c.is_whitespace()) {
            self.chars.next();
        }

        let ch = match self.chars.next() {
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
            '!' => match self.chars.peek() {
                Some('=') => {
                    self.chars.next();
                    Token::BangEqual
                }
                _ => Token::Bang,
            },
            '=' => match self.chars.peek() {
                Some('=') => {
                    self.chars.next();
                    Token::EqualEqual
                }
                _ => Token::Equal,
            },
            '<' => match self.chars.peek() {
                Some('=') => {
                    self.chars.next();
                    Token::LessEqual
                }
                _ => Token::Less,
            },
            '>' => match self.chars.peek() {
                Some('=') => {
                    self.chars.next();
                    Token::GreaterEqual
                }
                _ => Token::Greater,
            },
            _ => todo!(),
        };

        Some(token)
    }
}
