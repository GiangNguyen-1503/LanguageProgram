mod env;
mod typing;

use crate::typing::{Expr, Type, type_of};

fn main() {
    let expressions = [
        // 1 + IsZero(1)
        // Type mismatch: expected Int, found Bool
        Expr::Add(
            Box::new(Expr::Num(1)),
            Box::new(Expr::IsZero(Box::new(Expr::Num(1)))),
        ),
        // (fun f: (Int → Bool) -> (if (f 3) then 11 else 22))
        // Type: ((Int → Bool) → Int)
        Expr::Fun(
            "f".to_string(),
            Type::Fun(Box::new(Type::Int), Box::new(Type::Bool)),
            Box::new(Expr::IfThenElse(
                Box::new(Expr::App(
                    Box::new(Expr::Var("f".to_string())),
                    Box::new(Expr::Num(3)),
                )),
                Box::new(Expr::Num(11)),
                Box::new(Expr::Num(22)),
            )),
        ),
        // (let rec f (x: Int): Int = (if (iszero x) then 0 else (x + (f (x - 1)))) in f)
        // Type: (Int → Int)
        Expr::LetRecIn(
            "f".to_string(),
            "x".to_string(),
            Type::Int,
            Type::Int,
            Box::new(Expr::IfThenElse(
                Box::new(Expr::IsZero(Box::new(Expr::Var("x".to_string())))),
                Box::new(Expr::Num(0)),
                Box::new(Expr::Add(
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::App(
                        Box::new(Expr::Var("f".to_string())),
                        Box::new(Expr::Sub(
                            Box::new(Expr::Var("x".to_string())),
                            Box::new(Expr::Num(1)),
                        )),
                    )),
                )),
            )),
            Box::new(Expr::Var("f".to_string())),
        ),
    ];

    for expr in &expressions {
        println!("{expr}");
        match type_of(expr) {
            Ok(expr_type) => println!("Type: {expr_type}"),
            Err(e) => println!("{e}"),
        }
        println!("");
    }
}
