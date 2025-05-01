use crate::expr::{Expr, Literal};
use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug)]
pub enum Result {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

pub trait Eval {
    fn eval(&self) -> Result;
}

// The book allows things like truthiness for other types, but I can't abide that
impl Eval for Expr {
    fn eval(&self) -> Result {
        match self {
            Expr::Binary(lhs, binary_op, rhs) => {
                let rust_op = match binary_op {
                    crate::expr::BinaryOp::Equal => |lhs, rhs| Result::Bool(lhs == rhs),
                    crate::expr::BinaryOp::NotEqual => |lhs, rhs| Result::Bool(lhs != rhs),
                    crate::expr::BinaryOp::LessThan => |lhs, rhs| Result::Bool(lhs < rhs),
                    crate::expr::BinaryOp::LessEqual => |lhs, rhs| Result::Bool(lhs <= rhs),
                    crate::expr::BinaryOp::Greater => |lhs, rhs| Result::Bool(lhs > rhs),
                    crate::expr::BinaryOp::GreaterEqual => |lhs, rhs| Result::Bool(lhs >= rhs),
                    crate::expr::BinaryOp::Plus => |lhs, rhs| lhs + rhs,
                    crate::expr::BinaryOp::Minus => |lhs, rhs| lhs - rhs,
                    crate::expr::BinaryOp::Times => |lhs, rhs| lhs * rhs,
                    crate::expr::BinaryOp::Div => |lhs, rhs| lhs / rhs,
                };
                rust_op(lhs.eval(), rhs.eval())
            }
            Expr::Unary(unary_op, expr) => match (unary_op, expr.eval()) {
                (crate::expr::UnaryOp::Negate, Result::Number(n)) => Result::Number(-n),
                (crate::expr::UnaryOp::Not, Result::Bool(b)) => Result::Bool(!b),
                _ => panic!("Type error"),
            },
            Expr::Literal(literal) => literal.eval(),
            Expr::Grouping(expr) => expr.eval(),
        }
    }
}

impl Eval for Literal {
    fn eval(&self) -> Result {
        match self {
            Literal::Number(num) => Result::Number(*num),
            Literal::String(str) => Result::String(str.clone()),
            Literal::True => Result::Bool(true),
            Literal::False => Result::Bool(true),
            Literal::Nil => Result::Nil,
        }
    }
}

impl PartialEq for Result {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for Result {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl Add for Result {
    type Output = Result;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Result::Number(l), Result::Number(r)) => Result::Number(l + r),
            (Result::String(l), Result::String(r)) => Result::String(format!("{l}{r}")),
            _ => panic!("Type error, can't + those things"),
        }
    }
}

impl Sub for Result {
    type Output = Result;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Result::Number(l), Result::Number(r)) => Result::Number(l - r),
            _ => panic!("Type error, can't - those things"),
        }
    }
}

impl Mul for Result {
    type Output = Result;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Result::Number(l), Result::Number(r)) => Result::Number(l * r),
            _ => panic!("Type error, can't * those things"),
        }
    }
}

impl Div for Result {
    type Output = Result;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Result::Number(l), Result::Number(r)) => Result::Number(l / r),
            _ => panic!("Type error, can't / those things"),
        }
    }
}
