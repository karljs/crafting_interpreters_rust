use crate::token::Token;

pub struct Scanner {
    line: i32,
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { line: 0, source }
    }

    pub fn lexemes(&self) -> Lexemes {
        Lexemes {
            current: 0,
            characters: self.source.chars().collect(),
        }
    }
}

pub struct Lexemes {
    current: usize,
    characters: Vec<char>,
}

macro_rules! token {
    ( $self:expr, $token:expr, $num_chars:literal ) => {{
        $self.current += $num_chars;
        return Some($token);
    }};
}

impl Iterator for Lexemes {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current < self.characters.len() && self.characters[self.current].is_whitespace()
        {
            self.current += 1;
        }

        let chars_remaining = self.characters.len() - self.current - 1;

        if chars_remaining <= 0 {
            return Some(Token::EOF);
        }

        match self.characters[self.current..] {
            // check single-character tokens
            ['(', ..] => token!(self, Token::LeftParen, 1),
            [')', ..] => token!(self, Token::RightParen, 1),
            ['{', ..] => token!(self, Token::LeftBrace, 1),
            ['}', ..] => token!(self, Token::RightBrace, 1),
            [';', ..] => token!(self, Token::Semicolon, 1),
            [',', ..] => token!(self, Token::Comma, 1),
            ['.', ..] => token!(self, Token::Dot, 1),
            ['-', ..] => token!(self, Token::Minus, 1),
            ['+', ..] => token!(self, Token::Plus, 1),
            ['/', ..] => token!(self, Token::Slash, 1),
            ['*', ..] => token!(self, Token::Star, 1),

            // check 1-or-2-character tokens
            ['!', '=', ..] => token!(self, Token::BangEqual, 2),
            ['!', ..] => token!(self, Token::Bang, 1),
            ['=', '=', ..] => token!(self, Token::EqualEqual, 2),
            ['=', ..] => token!(self, Token::Equal, 1),
            ['<', '=', ..] => token!(self, Token::LessEqual, 2),
            ['<', ..] => token!(self, Token::Less, 1),
            ['>', '=', ..] => token!(self, Token::GreaterEqual, 2),
            ['>', ..] => token!(self, Token::Greater, 1),
            _ => {}
        }

        todo!()
    }
}
