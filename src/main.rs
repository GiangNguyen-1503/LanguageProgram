mod eval; 
mod env;

use crate::eval::Expr;
use crate::eval::Value;
use crate::eval::evaluate;

fn main() {
    // Example expressions
    let expr1 = Expr::Add(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)));
    let result1 = evaluate(&expr1);
    println!("2 + 3 = {:?}", result1); // Output: 2 + 3 = Int(5)

    let expr2 = Expr::IfThenElse(
        Box::new(Expr::IsZero(Box::new(Expr::Num(0)))),
        Box::new(Expr::Num(1)),
        Box::new(Expr::Num(0)),
    );
    let result2 = evaluate(&expr2);
    println!("if iszero(0) then 1 else 0 = {:?}", result2); // Output: if iszero(0) then 1 else 0 = Int(1)

    let expr3 = Expr::LetIn(String::from("x"),
        Box::new(Expr::Sub(Box::new(Expr::Num(5)), Box::new(Expr::Num(3)))),
        Box::new(Expr::Sub(Box::new(Expr::Var(String::from("x"))), Box::new(Expr::Num(1)))));
    let result3 = evaluate(&expr3);
    println!("let x = 5 - 3 in x - 1 = {:?}", result3);
}
