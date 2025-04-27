use crate::expr::{BinaryOp, Expr, Literal, UnaryOp, binop};
use crate::lexer::Token;

pub struct Parser {
    pub tokens: Vec<(Token, usize)>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, usize)>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.expr()
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

        match &self.tokens.get(self.current) {
            Some((Token::EqualEqual, _)) => {
                self.consume_cont_binop(lhs, BinaryOp::Equal, Parser::comparison)
            }
            Some((Token::BangEqual, _)) => {
                self.consume_cont_binop(lhs, BinaryOp::NotEqual, Parser::comparison)
            }
            _ => return lhs,
        }
    }

    fn comparison(&mut self) -> Expr {
        let lhs = self.term();

        match &self.tokens.get(self.current) {
            Some((Token::Less, _)) => {
                self.consume_cont_binop(lhs, BinaryOp::LessThan, Parser::term)
            }
            Some((Token::LessEqual, _)) => {
                self.consume_cont_binop(lhs, BinaryOp::LessEqual, Parser::term)
            }
            Some((Token::Greater, _)) => {
                self.consume_cont_binop(lhs, BinaryOp::Greater, Parser::term)
            }
            Some((Token::GreaterEqual, _)) => {
                self.consume_cont_binop(lhs, BinaryOp::GreaterEqual, Parser::term)
            }
            _ => return lhs,
        }
    }

    fn term(&mut self) -> Expr {
        let lhs = self.factor();

        match &self.tokens.get(self.current) {
            Some((Token::Plus, _)) => self.consume_cont_binop(lhs, BinaryOp::Plus, Parser::factor),
            Some((Token::Minus, _)) => {
                self.consume_cont_binop(lhs, BinaryOp::Minus, Parser::factor)
            }
            _ => return lhs,
        }
    }

    fn factor(&mut self) -> Expr {
        let lhs = self.unary();

        match &self.tokens.get(self.current) {
            Some((Token::Star, _)) => self.consume_cont_binop(lhs, BinaryOp::Times, Parser::unary),
            Some((Token::Slash, _)) => self.consume_cont_binop(lhs, BinaryOp::Div, Parser::unary),
            _ => return lhs,
        }
    }

    fn unary(&mut self) -> Expr {
        match &self.tokens.get(self.current) {
            Some((Token::Bang, _)) => self.consume_cont_unop(UnaryOp::Not, Parser::primary),
            Some((Token::Minus, _)) => self.consume_cont_unop(UnaryOp::Negate, Parser::primary),
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        match &self.tokens.get(self.current) {
            Some((Token::False, _)) => self.consume_literal(Literal::False),
            Some((Token::True, _)) => self.consume_literal(Literal::True),
            Some((Token::Nil, _)) => self.consume_literal(Literal::Nil),
            Some((Token::Number { literal, lexeme: _ }, _)) => {
                self.consume_literal(Literal::Number(*literal))
            }
            Some((Token::String { literal }, _)) => {
                self.consume_literal(Literal::String(literal.to_string()))
            }

            Some((Token::LeftParen, _)) => {
                self.current += 1;
                let expr = self.expr();
                self.current += 1;
                return Expr::Grouping(Box::new(expr));
            }
            _ => todo!(),
        }
    }
}
