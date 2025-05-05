use core::panic;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::{char, iter::Peekable, str::Chars};

use crate::scanner::Scanner;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
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
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("fun", TokenType::Fun),
            ("for", TokenType::For),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ])
    };
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
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

    pub fn tokens(mut self) -> Vec<Token> {
        let mut chars = self.scanner.scan().peekable();
        let mut tokens = Vec::new();
        let mut line = 1;

        while let Some(char) = chars.next() {
            let mut wrap_token = |t| {
                tokens.push(Token {
                    token_type: t,
                    line,
                    lexeme: char.to_string(),
                })
            };
            match char {
                '(' => wrap_token(TokenType::LeftParen),
                ')' => wrap_token(TokenType::RightParen),
                '{' => wrap_token(TokenType::LeftBrace),
                '}' => wrap_token(TokenType::RightBrace),
                ',' => wrap_token(TokenType::Comma),
                '.' => wrap_token(TokenType::Dot),
                '-' => wrap_token(TokenType::Minus),
                '+' => wrap_token(TokenType::Plus),
                ';' => wrap_token(TokenType::SemiColon),
                '*' => wrap_token(TokenType::Star),
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
        tokens.push(Token {
            token_type: TokenType::Eof,
            line,
            lexeme: String::new(),
        });
        return tokens;
    }
}

fn identifier_lookahead(
    char: char,
    chars: &mut Peekable<Chars<'_>>,
    tokens: &mut Vec<Token>,
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
        Some(token) => tokens.push(Token {
            token_type: token,
            line,
            lexeme,
        }),
        None => tokens.push(Token {
            token_type: TokenType::Identifier { name: identifier },
            line,
            lexeme,
        }),
    }
}

fn try_keyword(identifier: &str) -> Option<TokenType> {
    match KEYWORDS.get(identifier) {
        Some(token) => Some(token.clone()),
        None => None,
    }
}

fn number_lookahead(
    char: char,
    chars: &mut Peekable<Chars<'_>>,
    tokens: &mut Vec<Token>,
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
    tokens.push(Token {
        token_type: TokenType::Number { literal: number },
        line,
        lexeme,
    })
}

fn string_lookahead(
    chars: &mut Peekable<Chars<'_>>,
    tokens: &mut Vec<Token>,
    mut line: usize,
) -> usize {
    let mut string_val = String::new();
    loop {
        match chars.next() {
            Some('\n') => {
                line += 1;
            }
            Some('"') => {
                tokens.push(Token {
                    token_type: TokenType::String {
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
    tokens: &mut Vec<Token>,
    mut line: usize,
) -> usize {
    let mut push_token = |t| {
        tokens.push(Token {
            token_type: t,
            line,
            lexeme: char.to_string(),
        })
    };

    match (char, chars.peek()) {
        ('!', Some('=')) => {
            push_token(TokenType::BangEqual);
            chars.next();
        }
        ('!', _) => push_token(TokenType::Bang),
        ('=', Some('=')) => {
            push_token(TokenType::EqualEqual);
            chars.next();
        }
        ('=', _) => push_token(TokenType::Equal),
        ('<', Some('=')) => {
            push_token(TokenType::LessEqual);
            chars.next();
        }
        ('<', _) => push_token(TokenType::Less),
        ('>', Some('=')) => {
            push_token(TokenType::GreaterEqual);
            chars.next();
        }
        ('>', _) => push_token(TokenType::Greater),
        ('/', Some('/')) => {
            // consume comment
            chars.find(|x| *x == '\n');
            line += 1;
        }
        ('/', _) => push_token(TokenType::Slash),
        _ => {}
    }
    line
}
