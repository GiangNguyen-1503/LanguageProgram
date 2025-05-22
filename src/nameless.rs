use std::fmt;
use crate::named::NamedExpr;
use crate::nameless_env::NamelessEnv;

// Abstract syntax for expressions
#[derive(Debug, Clone)]
pub enum NamelessExpr {
    Num(i32),
    Var(i32),
    Add(Box<NamelessExpr>, Box<NamelessExpr>),
    Sub(Box<NamelessExpr>, Box<NamelessExpr>),
    IsZero(Box<NamelessExpr>),
    IfThenElse(Box<NamelessExpr>, Box<NamelessExpr>, Box<NamelessExpr>),
    LetIn(Box<NamelessExpr>, Box<NamelessExpr>),
    Fun(Box<NamelessExpr>),
    App(Box<NamelessExpr>, Box<NamelessExpr>),
}

impl fmt::Display for NamelessExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NamelessExpr::Num(n) => write!(f, "{}", n),
            NamelessExpr::Var(i) => write!(f, "#{}", i),
            NamelessExpr::Add(e1, e2) => write!(f, "{} + {}", e1, e2),
            NamelessExpr::Sub(e1, e2) => write!(f, "{} - {}", e1, e2),
            NamelessExpr::IsZero(e) => write!(f, "iszero {}", e),
            NamelessExpr::IfThenElse(e1, e2, e3) => write!(f, "(if {} then {} else {})", e1, e2, e3),
            NamelessExpr::LetIn(e1, e2) => write!(f, "(let {} in {})", e1, e2),
            NamelessExpr::Fun(body) => write!(f, "(fun {})", body),
            NamelessExpr::App(e1, e2) => write!(f, "({} {})", e1, e2),
        }
    }
}

// Convert NamedExpr to NamelessExpr
// Replace panic!("Not implemented") with your implementations.
// For error cases, you need to call panic!() with your own error message.
// For example, you should use panic!() when either of the operands of Add is not an integer.
pub fn to_nameless_inner(expr: &NamedExpr, env: &mut NamelessEnv<String>) -> NamelessExpr {
    match expr {
        NamedExpr::Num(n) => NamelessExpr::Num(*n),
        NamedExpr::Var(name) => {
            NamelessExpr::Var(env.index_of(name))
        },
        NamedExpr::Add(e1, e2) => {
            let e1 = to_nameless_inner(e1, env);
            let e2 = to_nameless_inner(e2, env);
            NamelessExpr::Add(Box::new(e1), Box::new(e2))
        },
        NamedExpr::Sub(e1, e2) => {
            let e1 = to_nameless_inner(e1, env);
            let e2 = to_nameless_inner(e2, env);
            NamelessExpr::Sub(Box::new(e1), Box::new(e2))
        },
        NamedExpr::IsZero(e) => {
            let e = to_nameless_inner(e, env);
            NamelessExpr::IsZero(Box::new(e))
        },
        NamedExpr::IfThenElse(c, t, e) => {
            let c = to_nameless_inner(c, env);
            let t = to_nameless_inner(t, env);
            let e = to_nameless_inner(e, env);
            NamelessExpr::IfThenElse(Box::new(c), Box::new(t), Box::new(e))
        },
        NamedExpr::LetIn(x, e1, e2) => {
            let e1 = to_nameless_inner(e1, env);
            let mut env = env.clone();
            env.insert(x.clone());
            let e2 = to_nameless_inner(e2, &mut env);
            NamelessExpr::LetIn(Box::new(e1), Box::new(e2))
        },
        NamedExpr::Fun(x, body) => {
            let mut env = env.clone();
            env.insert(x.clone());
            let body = to_nameless_inner(body, &mut env);
            NamelessExpr::Fun(Box::new(body))
        },
        NamedExpr::App(e1, e2) => {
            let e1 = to_nameless_inner(e1, env);
            let e2 = to_nameless_inner(e2, env);
            NamelessExpr::App(Box::new(e1), Box::new(e2))
        },
    }
}

pub fn to_nameless(expr: &NamedExpr) -> NamelessExpr {
    to_nameless_inner(expr, &mut NamelessEnv::new())
}
