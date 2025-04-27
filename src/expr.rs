use std::fmt;

// #[derive(Debug)]
// pub struct Expr(pub Equality);

// #[derive(Debug)]
// pub struct Equality {
//     comparison: Comparison,
//     rhs: Vec<(EqualityOp, Comparison)>,
// }

// #[derive(Debug)]
// pub enum EqualityOp {
//     NotEqual,
//     Equal,
// }

// #[derive(Debug)]
// pub struct Comparison {
//     term: Term,
//     rhs: Vec<(ComparisonOp, Term)>,
// }

// #[derive(Debug)]
// pub enum ComparisonOp {
//     Greater,
//     GreaterEqual,
//     Less,
//     LessEqual,
// }

// #[derive(Debug)]
// pub struct Term {
//     factor: Factor,
//     rhs: Vec<(TermOp, Factor)>,
// }

// #[derive(Debug)]
// pub enum TermOp {
//     Minus,
//     Plus,
// }

// #[derive(Debug)]
// pub struct Factor {
//     unary: Unary,
//     rhs: Vec<(FactorOp, Unary)>,
// }

// #[derive(Debug)]
// pub enum FactorOp {
//     Divide,
//     Times,
// }

// #[derive(Debug)]
// pub enum Unary {
//     Unary(UnaryOp, Vec<Box<Unary>>),
//     Primary,
// }

// #[derive(Debug)]
// pub enum UnaryOp {
//     Negate,
//     Not,
// }

// #[derive(Debug)]
// pub enum Primary {
//     Number(f64),
//     String(String),
//     True,
//     False,
//     Nil,
//     Parens(Expr),
// }

pub enum Expr {
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Literal(Literal),
    Grouping(Box<Expr>),
}

pub enum BinaryOp {
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Times,
    Div,
}

pub enum UnaryOp {
    Negate,
    Not,
}

pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            BinaryOp::LessThan => write!(f, "<"),
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
