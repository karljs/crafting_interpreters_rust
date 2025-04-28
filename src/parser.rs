use crate::expr::{BinaryOp, Expr, Literal, UnaryOp, binop};
use crate::lexer::{Token, TokenInfo};

pub struct Parser {
    pub tokens: Vec<TokenInfo>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.expr()
    }

    fn current_token(&mut self) -> Option<&Token> {
        self.tokens.get(self.current).map(|t| &t.token)
    }

    fn consume_cont(&mut self, f: fn(&mut Self) -> Expr, cc: impl FnOnce(Expr) -> Expr) -> Expr {
        self.current += 1;
        let rhs = f(self);
        cc(rhs)
    }

    fn consume_cont_binop(
        &mut self,
        lhs: Expr,
        op: BinaryOp,
        f_rhs: fn(&mut Self) -> Expr,
    ) -> Expr {
        self.consume_cont(f_rhs, |rhs| binop(lhs, op, rhs))
    }

    fn consume_cont_unop(&mut self, op: UnaryOp, f_rhs: fn(&mut Self) -> Expr) -> Expr {
        self.consume_cont(f_rhs, |rhs| Expr::Unary(op, Box::new(rhs)))
    }

    fn consume_literal(&mut self, lit: Literal) -> Expr {
        self.current += 1;
        Expr::Literal(lit)
    }

    fn expr(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let lhs = self.comparison();

        match self.current_token() {
            Some(Token::EqualEqual) => {
                self.consume_cont_binop(lhs, BinaryOp::Equal, Parser::comparison)
            }
            Some(Token::BangEqual) => {
                self.consume_cont_binop(lhs, BinaryOp::NotEqual, Parser::comparison)
            }
            _ => return lhs,
        }
    }

    fn comparison(&mut self) -> Expr {
        let lhs = self.term();

        match self.current_token() {
            Some(Token::Less) => self.consume_cont_binop(lhs, BinaryOp::LessThan, Parser::term),
            Some(Token::LessEqual) => {
                self.consume_cont_binop(lhs, BinaryOp::LessEqual, Parser::term)
            }
            Some(Token::Greater) => self.consume_cont_binop(lhs, BinaryOp::Greater, Parser::term),
            Some(Token::GreaterEqual) => {
                self.consume_cont_binop(lhs, BinaryOp::GreaterEqual, Parser::term)
            }
            _ => return lhs,
        }
    }

    fn term(&mut self) -> Expr {
        let lhs = self.factor();

        match self.current_token() {
            Some(Token::Plus) => self.consume_cont_binop(lhs, BinaryOp::Plus, Parser::factor),
            Some(Token::Minus) => self.consume_cont_binop(lhs, BinaryOp::Minus, Parser::factor),
            _ => return lhs,
        }
    }

    fn factor(&mut self) -> Expr {
        let lhs = self.unary();

        match self.current_token() {
            Some(Token::Star) => self.consume_cont_binop(lhs, BinaryOp::Times, Parser::unary),
            Some(Token::Slash) => self.consume_cont_binop(lhs, BinaryOp::Div, Parser::unary),
            _ => return lhs,
        }
    }

    fn unary(&mut self) -> Expr {
        match self.current_token() {
            Some(Token::Bang) => self.consume_cont_unop(UnaryOp::Not, Parser::primary),
            Some(Token::Minus) => self.consume_cont_unop(UnaryOp::Negate, Parser::primary),
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        match self.current_token() {
            Some(Token::False) => self.consume_literal(Literal::False),
            Some(Token::True) => self.consume_literal(Literal::True),
            Some(Token::Nil) => self.consume_literal(Literal::Nil),
            Some(Token::Number { literal }) => {
                let num = *literal;
                self.consume_literal(Literal::Number(num))
            }
            Some(Token::String { literal }) => {
                let text = literal.to_string();
                self.consume_literal(Literal::String(text))
            }

            Some(Token::LeftParen) => {
                self.current += 1;
                let expr = self.expr();
                self.current += 1;
                return Expr::Grouping(Box::new(expr));
            }
            _ => self.parse_error(),
        }
    }

    fn parse_error(&self) -> ! {
        let token_info = self.tokens.get(self.current).unwrap();
        panic!(
            "Parse error on line {:?}: {:?}",
            token_info.line, token_info.lexeme
        );
    }
}
