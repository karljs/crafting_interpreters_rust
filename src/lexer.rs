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

    Identifier { name: String },
    String { literal: String },
    Number { literal: f64 },

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

pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub lexeme: String,
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

    pub fn tokens(mut self) -> Vec<TokenInfo> {
        let mut chars = self.scanner.scan().peekable();
        let mut tokens = Vec::new();
        let mut line = 1;

        while let Some(char) = chars.next() {
            let mut wrap_token = |t| {
                tokens.push(TokenInfo {
                    token: t,
                    line,
                    lexeme: char.to_string(),
                })
            };
            match char {
                '(' => wrap_token(Token::LeftParen),
                ')' => wrap_token(Token::RightParen),
                '{' => wrap_token(Token::LeftBrace),
                '}' => wrap_token(Token::RightBrace),
                ',' => wrap_token(Token::Comma),
                '.' => wrap_token(Token::Dot),
                '-' => wrap_token(Token::Minus),
                '+' => wrap_token(Token::Plus),
                ';' => wrap_token(Token::SemiColon),
                '*' => wrap_token(Token::Star),
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
            };
        }
        tokens.push(TokenInfo {
            token: Token::Eof,
            line,
            lexeme: String::new(),
        });
        return tokens;
    }
}

fn identifier_lookahead(
    char: char,
    chars: &mut Peekable<Chars<'_>>,
    tokens: &mut Vec<TokenInfo>,
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
    let lexeme = identifier.to_string();
    match try_keyword(&identifier) {
        Some(token) => tokens.push(TokenInfo {
            token,
            line,
            lexeme,
        }),
        None => tokens.push(TokenInfo {
            token: Token::Identifier { name: identifier },
            line,
            lexeme,
        }),
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
    tokens: &mut Vec<TokenInfo>,
    line: usize,
) {
    let mut lexeme = char.to_string();
    while let Some(digit) = chars.peek() {
        if *digit == '.' || digit.is_numeric() {
            lexeme.push(*digit);
            chars.next();
        } else {
            break;
        }
    }
    let number: f64 = lexeme.parse().unwrap();
    tokens.push(TokenInfo {
        token: Token::Number { literal: number },
        line,
        lexeme,
    })
}

fn string_lookahead(
    chars: &mut Peekable<Chars<'_>>,
    tokens: &mut Vec<TokenInfo>,
    mut line: usize,
) -> usize {
    let mut string_val = String::new();
    loop {
        match chars.next() {
            Some('\n') => {
                line += 1;
            }
            Some('"') => {
                tokens.push(TokenInfo {
                    token: Token::String {
                        literal: string_val.to_string(),
                    },
                    line,
                    lexeme: string_val,
                });
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
    tokens: &mut Vec<TokenInfo>,
    mut line: usize,
) -> usize {
    let mut push_token = |t| {
        tokens.push(TokenInfo {
            token: t,
            line,
            lexeme: char.to_string(),
        })
    };

    match (char, chars.peek()) {
        ('!', Some('=')) => {
            push_token(Token::BangEqual);
            chars.next();
        }
        ('!', _) => push_token(Token::Bang),
        ('=', Some('=')) => {
            push_token(Token::EqualEqual);
            chars.next();
        }
        ('=', _) => push_token(Token::Equal),
        ('<', Some('=')) => {
            push_token(Token::LessEqual);
            chars.next();
        }
        ('<', _) => push_token(Token::Less),
        ('>', Some('=')) => {
            push_token(Token::GreaterEqual);
            chars.next();
        }
        ('>', _) => push_token(Token::Greater),
        ('/', Some('/')) => {
            // consume comment
            chars.find(|x| *x == '\n');
            line += 1;
        }
        ('/', _) => push_token(Token::Slash),
        _ => {}
    }
    line
}
