use std::fmt;

// Abstract syntax for NamedExpressions
#[derive(Debug, Clone)]
pub enum NamedExpr {
    Num(i32),
    Var(String),
    Add(Box<NamedExpr>, Box<NamedExpr>),
    Sub(Box<NamedExpr>, Box<NamedExpr>),
    IsZero(Box<NamedExpr>),
    IfThenElse(Box<NamedExpr>, Box<NamedExpr>, Box<NamedExpr>),
    LetIn(String, Box<NamedExpr>, Box<NamedExpr>),
    Fun(String, Box<NamedExpr>),
    App(Box<NamedExpr>, Box<NamedExpr>),
}

impl fmt::Display for NamedExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NamedExpr::Num(n) => write!(f, "{}", n),
            NamedExpr::Var(x) => write!(f, "{}", x),
            NamedExpr::Add(e1, e2) => write!(f, "{} + {}", e1, e2),
            NamedExpr::Sub(e1, e2) => write!(f, "{} - {}", e1, e2),
            NamedExpr::IsZero(e) => write!(f, "iszero {}", e),
            NamedExpr::IfThenElse(c, t, e) => write!(f, "(if {} then {} else {})", c, t, e),
            NamedExpr::LetIn(x, e1, e2) => write!(f, "(let {} = {} in {})", x, e1, e2),
            NamedExpr::Fun(x, body) => write!(f, "(fun {} -> {})", x, body),
            NamedExpr::App(e1, e2) => write!(f, "({} {})", e1, e2),
        }
    }
}
