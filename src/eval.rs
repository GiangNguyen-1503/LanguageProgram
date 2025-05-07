use crate::env::Env;

// Abstract syntax for expressions
#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    IsZero(Box<Expr>),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
    LetIn(String, Box<Expr>, Box<Expr>),
    Fun(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

// Abstract syntax for values
#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Closure(String, Box<Expr>, Box<Env>),
}

// Interpreter for expressions
// Replace panic!("Not implemented") with your implementation.
// For error cases, you need to call panic!() with your own error message.
// For example, you should use panic!() when either of the operands of Add is not an integer.
fn eval(expr: &Expr, env: &mut Env) -> Value {
    match expr {
        Expr::Num(n) => panic!("Not implemented"),
        Expr::Var(x) => panic!("Not implemented"),
        Expr::Add(e1, e2) => panic!("Not implemented"),
        Expr::Sub(e1, e2) => panic!("Not implemented"),
        Expr::IsZero(e) => panic!("Not implemented"),
        Expr::IfThenElse(e1, e2, e3) => panic!("Not implemented"),
        Expr::LetIn(x, e1, e2) => panic!("Not implemented"),
        Expr::Fun(x, e) => panic!("Not implemented"),
        Expr::App(e1, e2) => panic!("Not implemented"),
    }
}

pub fn evaluate(expr: &Expr) -> Value {
    eval(&expr, &mut Env::new())
}