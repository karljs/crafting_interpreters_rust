use crate::program::{BinaryOp, Expr, Literal, Program, Statement, UnaryOp};
use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, PartialEq)]
pub enum ExprEval {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    TypeError(String),
}

pub trait Eval {
    fn eval(&self) -> ExprEval;
}

impl Eval for Program {
    fn eval(&self) -> ExprEval {
        for stmt in self.into_iter() {
            stmt.eval();
        }
        ExprEval::Nil
    }
}

impl Eval for Statement {
    fn eval(&self) -> ExprEval {
        match self {
            Statement::Expr(expr) => {
                let res = expr.eval();
                println!("Evaluated to {:?}", res);
                res
            }
            Statement::Print(expr) => {
                let res = expr.eval();
                println!("Print called on: {:?}", res);
                res
            }
        }
    }
}

// The book allows things like truthiness for other types, but I can't abide that
impl Eval for Expr {
    fn eval(&self) -> ExprEval {
        match self {
            Expr::Binary(lhs, binary_op, rhs) => match (lhs.eval(), binary_op, rhs.eval()) {
                (lhs, BinaryOp::Equal, rhs) => ExprEval::Bool(lhs == rhs),
                (lhs, BinaryOp::NotEqual, rhs) => ExprEval::Bool(lhs != rhs),
                (lhs, BinaryOp::Less, rhs) => ExprEval::Bool(lhs < rhs),
                (lhs, BinaryOp::LessEqual, rhs) => ExprEval::Bool(lhs <= rhs),
                (lhs, BinaryOp::Greater, rhs) => ExprEval::Bool(lhs > rhs),
                (lhs, BinaryOp::GreaterEqual, rhs) => ExprEval::Bool(lhs >= rhs),
                (lhs, BinaryOp::Plus, rhs) => lhs + rhs,
                (lhs, BinaryOp::Minus, rhs) => lhs - rhs,
                (lhs, BinaryOp::Times, rhs) => lhs * rhs,
                (lhs, BinaryOp::Div, rhs) => lhs / rhs,
            },
            Expr::Unary(unary_op, expr) => match (unary_op, expr.eval()) {
                (UnaryOp::Negate, ExprEval::Number(n)) => ExprEval::Number(-n),
                (UnaryOp::Not, ExprEval::Bool(b)) => ExprEval::Bool(!b),
                (_, e @ ExprEval::TypeError(_)) => e,
                (_, e) => ExprEval::TypeError(format!("Can't apply {:?} to {:?}", unary_op, e)),
            },
            Expr::Literal(literal) => literal.eval(),
            Expr::Grouping(expr) => expr.eval(),
        }
    }
}

impl Eval for Literal {
    fn eval(&self) -> ExprEval {
        match self {
            Literal::Number(num) => ExprEval::Number(*num),
            Literal::String(str) => ExprEval::String(str.clone()),
            Literal::True => ExprEval::Bool(true),
            Literal::False => ExprEval::Bool(true),
            Literal::Nil => ExprEval::Nil,
        }
    }
}

impl PartialOrd for ExprEval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ExprEval::Number(l), ExprEval::Number(r)) => Some(l.total_cmp(r)),
            (ExprEval::String(l), ExprEval::String(r)) => Some(l.cmp(r)),
            _ => None,
        }
    }
}

impl Add for ExprEval {
    type Output = ExprEval;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ExprEval::Number(l), ExprEval::Number(r)) => ExprEval::Number(l + r),
            (ExprEval::String(l), ExprEval::String(r)) => ExprEval::String(format!("{l}{r}")),
            (l, r) => ExprEval::TypeError(format!("Can't add {:?} and {:?}", l, r)),
        }
    }
}

impl Sub for ExprEval {
    type Output = ExprEval;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ExprEval::Number(l), ExprEval::Number(r)) => ExprEval::Number(l - r),
            (l, r) => ExprEval::TypeError(format!("Can't subtract {:?} from {:?}", r, l)),
        }
    }
}

impl Mul for ExprEval {
    type Output = ExprEval;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ExprEval::Number(l), ExprEval::Number(r)) => ExprEval::Number(l * r),
            (l, r) => ExprEval::TypeError(format!("Can't multiply {:?} and {:?}", l, r)),
        }
    }
}

impl Div for ExprEval {
    type Output = ExprEval;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ExprEval::Number(l), ExprEval::Number(r)) => ExprEval::Number(l / r),
            (l, r) => ExprEval::TypeError(format!("Can't divide {:?} by {:?}", l, r)),
        }
    }
}
