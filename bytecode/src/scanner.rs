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

    fn offset(&mut self) -> usize {
        self.chars.peek().map_or(self.source.len(), |&(i, _)| i)
    }

    fn eat_while(&mut self, mut pred: impl FnMut(char) -> bool) -> &'a str {
        let start = self.offset();
        while self.chars.next_if(|&(_, c)| pred(c)).is_some() {}
        &self.source[start..self.offset()]
    }

    fn skip_whitespace(&mut self) -> bool {
        let ws = self.eat_while(char::is_whitespace);
        self.line += ws.matches('\n').count() as u32;
        !ws.is_empty()
    }

    fn skip_comment(&mut self) -> bool {
        match self.chars.peek() {
            Some(&(i, '/')) if self.source[i..].starts_with("//") => {
                self.eat_while(|c| c != '\n');
                true
            }
            _ => false,
        }
    }

    fn skip_ignored(&mut self) {
        while self.skip_whitespace() || self.skip_comment() {}
    }

    fn string(&mut self) -> Token<'a> {
        let body = self.eat_while(|c| c != '"');
        self.line += body.matches('\n').count() as u32;

        match self.chars.next() {
            Some(_) => Token::String(body),
            None => panic!("unterminated string on line {}", self.line + 1),
        }
    }

    fn number(&mut self, start: usize) -> Token<'a> {
        self.eat_while(|c| c.is_ascii_digit());

        // fractional part: consume '.' only when a digit follows it
        if let Some(&(dot, '.')) = self.chars.peek()
            && self.source[dot + 1..].starts_with(|c: char| c.is_ascii_digit())
        {
            self.chars.next();
            self.eat_while(|c| c.is_ascii_digit());
        }

        Token::Number(&self.source[start..self.offset()])
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
            '"' => self.string(),
            '0'..='9' => self.number(i),
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
