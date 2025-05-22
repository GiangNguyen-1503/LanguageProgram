mod named;
mod nameless_env;
mod nameless;

use crate::named::NamedExpr;

fn main() {
    let expressions = [
        // ((fun x -> x - 1) (let y = 5 in y + 2))
        NamedExpr::App(
            Box::new(NamedExpr::Fun(
                "x".to_string(),
                Box::new(NamedExpr::Sub(
                    Box::new(NamedExpr::Var("x".to_string())),
                    Box::new(NamedExpr::Num(1)),
                )),
            )),
            Box::new(NamedExpr::LetIn(
                "y".to_string(),
                Box::new(NamedExpr::Num(5)),
                Box::new(NamedExpr::Add(
                    Box::new(NamedExpr::Var("y".to_string())),
                    Box::new(NamedExpr::Num(2)),
                )),
            )),
        ),
        // (let x = 1 in (let y = 2 in x + y) + x)
        NamedExpr::LetIn(
            "x".to_string(),
            Box::new(NamedExpr::Num(1)),
            Box::new(NamedExpr::Add(
                Box::new(NamedExpr::LetIn(
                    "y".to_string(),
                    Box::new(NamedExpr::Num(2)),
                    Box::new(NamedExpr::Add(
                        Box::new(NamedExpr::Var("x".to_string())),
                        Box::new(NamedExpr::Var("y".to_string())),
                    )),
                )),
                Box::new(NamedExpr::Var("x".to_string())),
            )),
        ),
    ];

    for named_expr in &expressions {
        println!("{named_expr}");
        let nameless_expr = nameless::to_nameless(named_expr);
        println!("{nameless_expr}");
        println!("");
    }
}
