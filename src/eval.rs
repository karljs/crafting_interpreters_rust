use crate::expr::{BinaryOp, Expr, Literal};
use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    TypeError(String),
}

pub trait Eval {
    fn eval(&self) -> EvalResult;
}

// The book allows things like truthiness for other types, but I can't abide that
impl Eval for Expr {
    fn eval(&self) -> EvalResult {
        match self {
            Expr::Binary(lhs, binary_op, rhs) => match (lhs.eval(), binary_op, rhs.eval()) {
                (lhs, BinaryOp::Equal, rhs) => EvalResult::Bool(lhs == rhs),
                (lhs, BinaryOp::NotEqual, rhs) => EvalResult::Bool(lhs != rhs),
                (lhs, BinaryOp::LessThan, rhs) => EvalResult::Bool(lhs < rhs),
                (lhs, BinaryOp::LessEqual, rhs) => EvalResult::Bool(lhs <= rhs),
                (lhs, BinaryOp::Greater, rhs) => EvalResult::Bool(lhs > rhs),
                (lhs, BinaryOp::GreaterEqual, rhs) => EvalResult::Bool(lhs >= rhs),
                (lhs, BinaryOp::Plus, rhs) => lhs + rhs,
                (lhs, BinaryOp::Minus, rhs) => lhs - rhs,
                (lhs, BinaryOp::Times, rhs) => lhs * rhs,
                (lhs, BinaryOp::Div, rhs) => lhs / rhs,
            },
            Expr::Unary(unary_op, expr) => match (unary_op, expr.eval()) {
                (crate::expr::UnaryOp::Negate, EvalResult::Number(n)) => EvalResult::Number(-n),
                (crate::expr::UnaryOp::Not, EvalResult::Bool(b)) => EvalResult::Bool(!b),
                (_, e @ EvalResult::TypeError(_)) => e,
                (_, e) => EvalResult::TypeError(format!("Can't apply {:?} to {:?}", unary_op, e)),
            },
            Expr::Literal(literal) => literal.eval(),
            Expr::Grouping(expr) => expr.eval(),
        }
    }
}

impl Eval for Literal {
    fn eval(&self) -> EvalResult {
        match self {
            Literal::Number(num) => EvalResult::Number(*num),
            Literal::String(str) => EvalResult::String(str.clone()),
            Literal::True => EvalResult::Bool(true),
            Literal::False => EvalResult::Bool(true),
            Literal::Nil => EvalResult::Nil,
        }
    }
}

impl PartialOrd for EvalResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (EvalResult::Number(l), EvalResult::Number(r)) => Some(l.total_cmp(r)),
            (EvalResult::String(l), EvalResult::String(r)) => Some(l.cmp(r)),
            _ => None,
        }
    }
}

impl Add for EvalResult {
    type Output = EvalResult;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EvalResult::Number(l), EvalResult::Number(r)) => EvalResult::Number(l + r),
            (EvalResult::String(l), EvalResult::String(r)) => EvalResult::String(format!("{l}{r}")),
            (l, r) => EvalResult::TypeError(format!("Can't add {:?} and {:?}", l, r)),
        }
    }
}

impl Sub for EvalResult {
    type Output = EvalResult;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EvalResult::Number(l), EvalResult::Number(r)) => EvalResult::Number(l - r),
            (l, r) => EvalResult::TypeError(format!("Can't subtract {:?} from {:?}", r, l)),
        }
    }
}

impl Mul for EvalResult {
    type Output = EvalResult;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EvalResult::Number(l), EvalResult::Number(r)) => EvalResult::Number(l * r),
            (l, r) => EvalResult::TypeError(format!("Can't multiply {:?} and {:?}", l, r)),
        }
    }
}

impl Div for EvalResult {
    type Output = EvalResult;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EvalResult::Number(l), EvalResult::Number(r)) => EvalResult::Number(l / r),
            (l, r) => EvalResult::TypeError(format!("Can't divide {:?} by {:?}", l, r)),
        }
    }
}
