//! Encodes the AST of the language

use std::fmt;

use crate::lexer::Token;

pub type Program = Vec<Declaration>;

#[derive(Debug)]
pub enum Declaration {
    Variable {
        identifier: String,
        value: Option<Expr>,
    },
    Statement(Statement),
}

#[derive(Debug)]
pub enum Statement {
    Expr(Expr),
    Print(Expr),
}

#[derive(Clone)]
pub enum Expr {
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Assignment(Token, Box<Expr>),
    Literal(Literal),
    Grouping(Box<Expr>),
}

#[derive(Clone)]
pub enum BinaryOp {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Times,
    Div,
}

#[derive(Clone)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Identifier(String),
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Assignment(lhs, rhs) => write!(f, "{:?} = {:?}", lhs, rhs),
            Expr::Literal(literal) => write!(f, "{:?}", literal),
            Expr::Unary(unary_op, expr) => write!(f, "({:?} {:?})", unary_op, expr),
            Expr::Binary(lhs, binary_op, rhs) => {
                write!(f, "({:?} {:?} {:?})", binary_op, lhs, rhs)
            }
            Expr::Grouping(expr) => write!(f, "(group {:?})", expr),
        }
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(number) => write!(f, "{}", number),
            Literal::String(string) => write!(f, "{}", string),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Nil => write!(f, "nil"),
            Literal::Identifier(id) => write!(f, "{}", id),
        }
    }
}

impl fmt::Debug for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Negate => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

impl fmt::Debug for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Equal => write!(f, "=="),
            BinaryOp::NotEqual => write!(f, "!="),
            BinaryOp::Less => write!(f, "<"),
            BinaryOp::LessEqual => write!(f, "<="),
            BinaryOp::Greater => write!(f, ">"),
            BinaryOp::GreaterEqual => write!(f, ">="),
            BinaryOp::Plus => write!(f, "+"),
            BinaryOp::Minus => write!(f, "-"),
            BinaryOp::Times => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
        }
    }
}

pub fn binop(lhs: Expr, binop: BinaryOp, rhs: Expr) -> Expr {
    Expr::Binary(Box::new(lhs), binop, Box::new(rhs))
}
