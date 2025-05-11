use crate::{
    environment::{self, Environment},
    error::runtime_error,
    lexer::TokenType,
    program::{BinaryOp, Declaration, Expr, Literal, Program, Statement, UnaryOp},
};
use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Debug, PartialEq)]
pub enum ExprEval {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    RuntimeError(String),
    TypeError(String),
}

pub trait Eval {
    fn eval(&self, environment: &mut Environment) -> ExprEval;
}

impl Eval for Program {
    fn eval(&self, environment: &mut Environment) -> ExprEval {
        for decl in self.into_iter() {
            decl.eval(environment);
        }
        ExprEval::Nil
    }
}

impl Eval for Declaration {
    fn eval(&self, environment: &mut Environment) -> ExprEval {
        match self {
            Declaration::Variable { identifier, value } => {
                if let Some(expr) = value {
                    let rhs = expr.eval(environment);
                    environment.define(identifier.clone(), Some(rhs));
                } else {
                    environment.define(identifier.clone(), None);
                }
                println!("Declared variable {:?}", identifier);
                environment.debug_dump();
                ExprEval::Nil
            }
            Declaration::Statement(statement) => statement.eval(environment),
        }
    }
}

impl Eval for Statement {
    fn eval(&self, environment: &mut Environment) -> ExprEval {
        match self {
            Statement::Expr(expr) => {
                let res = expr.eval(environment);
                println!("Evaluated to {:?}", res);
                res
            }
            Statement::Print(expr) => {
                let res = expr.eval(environment);
                println!("Print called on: {:?}", res);
                res
            }
        }
    }
}

// The book allows things like truthiness for other types, but I can't abide that
impl Eval for Expr {
    fn eval(&self, environment: &mut Environment) -> ExprEval {
        match self {
            Expr::Assignment(lhs, rhs) => match (&lhs.token_type, rhs.eval(environment)) {
                (TokenType::Identifier { name }, rhs) => {
                    if environment.get(name).is_ok() {
                        environment.define(name.clone(), Some(rhs.clone()));
                        rhs
                    } else {
                        ExprEval::RuntimeError(format!("Assigned to unknown variable {:?}", name))
                    }
                }
                _ => ExprEval::RuntimeError("Illegal assignment".to_string()),
            },
            Expr::Binary(lhs, binary_op, rhs) => {
                match (lhs.eval(environment), binary_op, rhs.eval(environment)) {
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
                }
            }
            Expr::Unary(unary_op, expr) => match (unary_op, expr.eval(environment)) {
                (UnaryOp::Negate, ExprEval::Number(n)) => ExprEval::Number(-n),
                (UnaryOp::Not, ExprEval::Bool(b)) => ExprEval::Bool(!b),
                (_, e @ ExprEval::TypeError(_)) => e,
                (_, e) => ExprEval::TypeError(format!("Can't apply {:?} to {:?}", unary_op, e)),
            },
            Expr::Literal(literal) => literal.eval(environment),
            Expr::Grouping(expr) => expr.eval(environment),
        }
    }
}

impl Eval for Literal {
    fn eval(&self, environment: &mut Environment) -> ExprEval {
        match self {
            Literal::Number(num) => ExprEval::Number(*num),
            Literal::String(str) => ExprEval::String(str.clone()),
            Literal::True => ExprEval::Bool(true),
            Literal::False => ExprEval::Bool(false),
            Literal::Nil => ExprEval::Nil,
            Literal::Identifier(id) => {
                println!("variable access {:?}", id);
                environment.debug_dump();

                match environment.get(id) {
                    Ok(Some(res)) => res.clone(),
                    Ok(None) => ExprEval::RuntimeError(format!(
                        "Variable {:?} accessed before definition",
                        id
                    )),

                    Err(_) => ExprEval::RuntimeError(format!("Unknown variable {:?}", id)),
                }
            }
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
