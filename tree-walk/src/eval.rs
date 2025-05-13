use crate::{
    environment::Environment,
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
    RuntimeTypeError(String),
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
            Statement::Block(statements) => {
                environment.enter_scope();
                for stmt in statements {
                    stmt.eval(environment);
                }
                environment.exit_scope();
                ExprEval::Nil
            }
            Statement::IfElse(cond, then_branch, else_branch) => match cond.eval(environment) {
                ExprEval::Bool(b) => {
                    if b {
                        return then_branch.eval(environment);
                    } else {
                        if let Some(else_branch) = &**else_branch {
                            return else_branch.eval(environment);
                        }
                        return ExprEval::Nil;
                    }
                }
                _ => ExprEval::RuntimeTypeError(
                    "Expression in conditional didn't evaluate to a bool".to_string(),
                ),
            },
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
                        environment.assign(name.clone(), Some(rhs.clone()));
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
                (_, e @ ExprEval::RuntimeTypeError(_)) => e,
                (_, e) => {
                    ExprEval::RuntimeTypeError(format!("Can't apply {:?} to {:?}", unary_op, e))
                }
            },
            Expr::Literal(literal) => literal.eval(environment),
            Expr::Grouping(expr) => expr.eval(environment),
            Expr::Logical(lhs, logical_op, rhs) => {
                // as with many things, this diverges from the book
                // because I don't really want to implement implicit
                // "thruthiness"
                match (lhs.eval(environment), logical_op, rhs.eval(environment)) {
                    (ExprEval::Bool(lhs), crate::program::LogicalOp::Or, ExprEval::Bool(rhs)) => {
                        return ExprEval::Bool(lhs || rhs);
                    }
                    (ExprEval::Bool(lhs), crate::program::LogicalOp::And, ExprEval::Bool(rhs)) => {
                        return ExprEval::Bool(lhs && rhs);
                    }
                    _ => ExprEval::RuntimeTypeError(
                        "Logical operator applied to non-boolean value".to_string(),
                    ),
                }
            }
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
            (l, r) => ExprEval::RuntimeTypeError(format!("Can't add {:?} and {:?}", l, r)),
        }
    }
}

impl Sub for ExprEval {
    type Output = ExprEval;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ExprEval::Number(l), ExprEval::Number(r)) => ExprEval::Number(l - r),
            (l, r) => ExprEval::RuntimeTypeError(format!("Can't subtract {:?} from {:?}", r, l)),
        }
    }
}

impl Mul for ExprEval {
    type Output = ExprEval;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ExprEval::Number(l), ExprEval::Number(r)) => ExprEval::Number(l * r),
            (l, r) => ExprEval::RuntimeTypeError(format!("Can't multiply {:?} and {:?}", l, r)),
        }
    }
}

impl Div for ExprEval {
    type Output = ExprEval;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ExprEval::Number(l), ExprEval::Number(r)) => ExprEval::Number(l / r),
            (l, r) => ExprEval::RuntimeTypeError(format!("Can't divide {:?} by {:?}", l, r)),
        }
    }
}
