use crate::parse::*;
use std::collections::HashMap;

pub fn eval(expr: Expr, context: &mut HashMap<String, Expr>) {
    match expr {
        Expr::Call(name, arg) => {
            if name == "println!" {
                println!("{arg}");
            }
        }
        Expr::Let(name, value) => context.insert(name, value),
        Expr::Constant(atom) => todo!(),
    }
}
