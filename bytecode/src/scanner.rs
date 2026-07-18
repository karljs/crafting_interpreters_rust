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
            chars: self.source.char_indices().peekable(),
            line: 0,
            done: false,
        }
    }
}

pub struct Lexemes<'a> {
    chars: Peekable<CharIndices<'a>>,
    line: u32,
    done: bool,
}

impl Lexemes<'_> {
    fn consume_if(&mut self, expected: char) -> bool {
        self.chars.next_if(|&(_, c)| c == expected).is_some()
    }
}

impl<'a> Iterator for Lexemes<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((_, c)) = self.chars.next_if(|&(_, c)| c.is_whitespace()) {
            if c == '\n' {
                self.line += 1;
            }
        }

        let (_, ch) = match self.chars.next() {
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
