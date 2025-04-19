use crate::parse::*;
use std::collections::HashMap;

pub fn eval(expr: Expr, context: &mut HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Call(name, arg) => {
            let arg = eval(*arg, context);
            if name == "println!" {
                println!("{arg:?}");
            }
            arg
        }
        Expr::Let(name, value) => {
            context.insert(name, *value);
            Expr::Void
        }
        Expr::Constant(ref atom) => match atom {
            Atom::Name(name) => context.get(name).unwrap().clone(),
            _ => expr,
        },
        Expr::Void => expr,
    }
}
