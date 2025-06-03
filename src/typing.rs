use std::fmt;
use crate::env::TypeEnv;

// Abstract syntax for Expressions
#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    IsZero(Box<Expr>),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
    LetIn(String, Box<Expr>, Box<Expr>),
    LetRecIn(String, String, Type, Type, Box<Expr>, Box<Expr>),
    Fun(String, Type, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Var(x) => write!(f, "{x}"),
            Expr::Add(e1, e2) => write!(f, "({e1} + {e2})"),
            Expr::Sub(e1, e2) => write!(f, "({e1} - {e2})"),
            Expr::IsZero(e) => write!(f, "(iszero {e})"),
            Expr::IfThenElse(cond, t_branch, f_branch) => write!(f, "(if {cond} then {t_branch} else {f_branch})"),
            Expr::LetIn(x, e1, e2) => write!(f, "(let {x} = {e1} in {e2})"),
            Expr::LetRecIn(func_name, arg_name, arg_type, body_type, body, e) => write!(f, "(let rec {func_name} ({arg_name}: {arg_type}): {body_type} = {body} in {e})"),
            Expr::Fun(param, param_type, body) => write!(f, "(fun {param}: {param_type} -> {body})"),
            Expr::App(func, e) => write!(f, "({func} {e})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Fun(Box<Type>, Box<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Bool => write!(f, "Bool"),
            Type::Fun(param_t, ret_t) => write!(f, "({} → {})", param_t, ret_t),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeError {
    TypeMismatch(Type, Type),
    NotAFunction(Type),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::TypeMismatch(expected, found) => write!(f,
                "Type mismatch: expected {}, found {}", expected, found),
            TypeError::NotAFunction(t) => write!(f, "Not a function type: {}", t),
        }
    }
}

// Type check a given expression in the current type environment.
// Replace panic!("Not implemented") with your type-checking logic.
// For each case, return Ok(Type) if the expression is well-typed,
// or return Err(TypeError::...) with appropriate error messages.
// 
// For example, you should return Err(TypeError::TypeMismatch(...))
// when the operands of Add do not have type Int, or
// Err(TypeError::NotAFunction(...)) if a non-function is applied.
fn type_check(expr: &Expr, env: &mut TypeEnv) -> Result<Type, TypeError> {
    match expr {
        Expr::Num(_) => Ok(Type::Int),
        Expr::Var(x) => {
            match env.lookup(x) {
                t => Ok(t),
            }
        },
        Expr::Add(e1, e2) => {
            let t1 = type_check(e1, env)?;
            let t2 = type_check(e2, env)?;
            if t1 == Type::Int && t2 == Type::Int {
                Ok(Type::Int)
            } else {
                Err(TypeError::TypeMismatch(Type::Int, if t1 != Type::Int { t1 } else { t2 }))
            }
        },
        Expr::Sub(e1, e2) => {
            let t1 = type_check(e1, env)?;
            let t2 = type_check(e2, env)?;
            if t1 == Type::Int && t2 == Type::Int {
                Ok(Type::Int)
            } else {
                Err(TypeError::TypeMismatch(Type::Int, if t1 != Type::Int { t1 } else { t2 }))
            }
        },
        Expr::IsZero(e) => {
            let t = type_check(e, env)?;
            if t == Type::Int {
                Ok(Type::Bool)
            } else {
                Err(TypeError::TypeMismatch(Type::Int, t))
            }
        },
        Expr::IfThenElse(cond, t_branch, f_branch) => {
            let t_cond = type_check(cond, env)?;
            if t_cond != Type::Bool {
                return Err(TypeError::TypeMismatch(Type::Bool, t_cond))
            }
            let t1 = type_check(t_branch, env)?;
            let t2 = type_check(f_branch, env)?;
            if t1 == t2 {
                Ok(t1)
            } else {
                Err(TypeError::TypeMismatch(t1, t2))
            }
        },
        Expr::LetIn(x, e1, e2) => {
            let t1 = type_check(e1, env)?;
            env.insert(x.clone(), t1);
            let t2 = type_check(e2, env)?;
            Ok(t2)
        },
        Expr::LetRecIn(func_name, arg_name, arg_type, body_type, body, e) => {
            let fun_type = Type::Fun(Box::new(arg_type.clone()), Box::new(body_type.clone()));
            env.insert(func_name.clone(), fun_type.clone());
            env.insert(arg_name.clone(), arg_type.clone());
            let t_body = type_check(body, env)?;
            if t_body != *body_type {
                return Err(TypeError::TypeMismatch(body_type.clone(), t_body));
            }
            let t_e = type_check(e, env)?;
            Ok(t_e)
        },
        Expr::Fun(param, param_type, body) => {
            env.insert(param.clone(), param_type.clone());
            let t_body = type_check(body, env)?;
            Ok(Type::Fun(Box::new(param_type.clone()), Box::new(t_body)))
        },
        Expr::App(func, e) => {
            let t_func = type_check(func, env)?;
            let t_arg = type_check(e, env)?;
            match t_func {
                Type::Fun(param_type, return_type) => {
                    if *param_type == t_arg {
                        Ok(*return_type)
                    } else {
                        Err(TypeError::TypeMismatch(*param_type, t_arg))
                    }
                },
                _ => Err(TypeError::NotAFunction(t_func)),
            }
        }
    }
}

pub fn type_of(expr: &Expr) -> Result<Type, TypeError> {
    return type_check(expr, &mut TypeEnv::new());
}
