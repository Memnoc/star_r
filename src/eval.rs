use crate::parse::*;
use std::collections::HashMap;

pub fn eval(expr: Expr, context: &mut HashMap<String, Atom>) {
    match expr {
        Expr::Call(name, arg) => {
            if name == "println!" {
                println!("{arg}");
            }
        }
        Expr::Let(name, value) => {
            context.insert(name, value);
        }
    }
}
