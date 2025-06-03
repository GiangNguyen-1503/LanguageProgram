use crate::env::Env;

// Abstract syntax for expressions
#[derive(Debug, Clone)]
#[allow(dead_code)]
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
        Expr::Num(n) => Value::Int(*n),
        Expr::Var(x) => env.lookup(x),
        Expr::Add(e1, e2) => {
            match (eval(e1, env), eval(e2, env)) {
                (Value::Int(i1), Value::Int(i2)) => Value::Int(i1 + i2),
                _ => panic!("Evaluation Error!"),
            }
        }
        Expr::Sub(e1, e2) => {
            match (eval(e1, env), eval(e2, env)) {
                (Value::Int(i1), Value::Int(i2)) => Value::Int(i1 - i2),
                _ => panic!("Evaluation Error!"),
            }
        },
        Expr::IsZero(e) => {
            match eval(e, env) {
                Value::Int(i) => Value::Bool(i == 0),
                _ => panic!("Evaluation Error!"),
            }
        },
        Expr::IfThenElse(e1, e2, e3) => {
            match eval(e1, env) {
                Value::Bool(true) => eval(e2, env),
                Value::Bool(false) => eval(e3, env),
                _ => panic!("Evaluation Error!"),
            }
        },
        Expr::LetIn(x, e1, e2) => {
            let value = eval(e1, env);
            env.insert(x.clone(), value);
            let result = eval(e2, env);
            env.remove(x);
            result
        },
        Expr::Fun(x, e) => {
            Value::Closure(x.clone(), e.clone(), Box::new(env.clone()))
        },
        Expr::App(e1, e2) => {
            match eval(e1, env) {
                Value::Closure(x, e, mut env_closure) => {
                    env_closure.insert(x, eval(e2, env));
                    eval(&e, &mut env_closure)
                }
                _ => panic!("Evaluation Error!"),
            }
        },
    }
}

pub fn evaluate(expr: &Expr) -> Value {
    eval(&expr, &mut Env::new())
}