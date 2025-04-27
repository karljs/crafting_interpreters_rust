use core::panic;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::{char, iter::Peekable, str::Chars};

use crate::scanner::Scanner;

#[derive(Clone, Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier { lexeme: String },
    String { literal: String },
    Number { literal: f64, lexeme: String },

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = {
        HashMap::from([
            ("and", Token::And),
            ("class", Token::Class),
            ("else", Token::Else),
            ("false", Token::False),
            ("fun", Token::Fun),
            ("for", Token::For),
            ("if", Token::If),
            ("nil", Token::Nil),
            ("or", Token::Or),
            ("print", Token::Print),
            ("return", Token::Return),
            ("super", Token::Super),
            ("this", Token::This),
            ("true", Token::True),
            ("var", Token::Var),
            ("while", Token::While),
        ])
    };
}

pub struct Lexer {
    scanner: Scanner,
    errors: Vec<String>,
}

impl Lexer {
    pub fn from_source(source: String) -> Self {
        Lexer {
            scanner: Scanner::from_source(source),
            errors: Vec::new(),
        }
    }

    pub fn tokens(mut self) -> Vec<(Token, usize)> {
        let mut chars = self.scanner.scan().peekable();
        let mut tokens = Vec::new();
        let mut line = 1;

        while let Some(char) = chars.next() {
            match char {
                '(' => tokens.push((Token::LeftParen, line)),
                ')' => tokens.push((Token::RightParen, line)),
                '{' => tokens.push((Token::LeftBrace, line)),
                '}' => tokens.push((Token::RightBrace, line)),
                ',' => tokens.push((Token::Comma, line)),
                '.' => tokens.push((Token::Dot, line)),
                '-' => tokens.push((Token::Minus, line)),
                '+' => tokens.push((Token::Plus, line)),
                ';' => tokens.push((Token::SemiColon, line)),
                '*' => tokens.push((Token::Star, line)),
                '!' | '=' | '<' | '>' | '/' => {
                    line = symbol_lookahead(char, &mut chars, &mut tokens, line);
                }
                '\n' => line += 1,

                '"' => {
                    line = string_lookahead(&mut chars, &mut tokens, line);
                }
                char if char.is_numeric() => {
                    number_lookahead(char, &mut chars, &mut tokens, line);
                }
                char if char.is_alphabetic() => {
                    identifier_lookahead(char, &mut chars, &mut tokens, line);
                }
                char if char.is_ascii_whitespace() => {}

                _ => self.errors.push("error".to_string()),
            }
        }
        return tokens;
    }
}

fn identifier_lookahead(
    char: char,
    chars: &mut Peekable<Chars<'_>>,
    tokens: &mut Vec<(Token, usize)>,
    line: usize,
) {
    let mut identifier = char.to_string();
    while let Some(c) = chars.peek() {
        if *c == '_' || c.is_alphanumeric() {
            identifier.push(*c);
            chars.next();
        } else {
            break;
        }
    }
    match try_keyword(&identifier) {
        Some(token) => tokens.push((token, line)),
        None => tokens.push((Token::Identifier { lexeme: identifier }, line)),
    }
}

fn try_keyword(identifier: &str) -> Option<Token> {
    match KEYWORDS.get(identifier) {
        Some(token) => Some(token.clone()),
        None => None,
    }
}

fn number_lookahead(
    char: char,
    chars: &mut Peekable<Chars<'_>>,
    tokens: &mut Vec<(Token, usize)>,
    line: usize,
) {
    let mut number_str = char.to_string();
    while let Some(digit) = chars.peek() {
        if *digit == '.' || digit.is_numeric() {
            number_str.push(*digit);
            chars.next();
        } else {
            break;
        }
    }
    let number: f64 = number_str.parse().unwrap();
    tokens.push((
        Token::Number {
            literal: number,
            lexeme: number_str,
        },
        line,
    ));
}

fn string_lookahead(
    chars: &mut Peekable<Chars<'_>>,
    tokens: &mut Vec<(Token, usize)>,
    mut line: usize,
) -> usize {
    let mut string_val = String::new();
    loop {
        match chars.next() {
            Some('\n') => {
                line += 1;
            }
            Some('"') => {
                tokens.push((
                    Token::String {
                        literal: string_val,
                    },
                    line,
                ));
                break;
            }
            Some(c) => {
                string_val.push(c);
            }
            None => {
                panic!("Error")
            }
        }
    }
    return line;
}

fn symbol_lookahead(
    char: char,
    chars: &mut Peekable<Chars<'_>>,
    tokens: &mut Vec<(Token, usize)>,
    mut line: usize,
) -> usize {
    match (char, chars.peek()) {
        ('!', Some('=')) => {
            tokens.push((Token::BangEqual, line));
            chars.next();
        }
        ('!', _) => tokens.push((Token::Bang, line)),
        ('=', Some('=')) => {
            tokens.push((Token::EqualEqual, line));
            chars.next();
        }
        ('=', _) => tokens.push((Token::Equal, line)),
        ('<', Some('=')) => {
            tokens.push((Token::LessEqual, line));
            chars.next();
        }
        ('<', _) => tokens.push((Token::Less, line)),
        ('>', Some('=')) => {
            tokens.push((Token::GreaterEqual, line));
            chars.next();
        }
        ('>', _) => tokens.push((Token::Greater, line)),
        ('/', Some('/')) => {
            // consume comment
            chars.find(|x| *x == '\n');
            line += 1;
        }
        ('/', _) => tokens.push((Token::Slash, line)),
        _ => {}
    }
    line
}
