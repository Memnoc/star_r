use crate::parse::*;
use std::collections::HashMap;

pub fn eval(expr: Expr, context: &mut HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Call(name, arg) => {
            if name == "println!" {
                let arg = eval(*arg, context);
                println!("{arg:?}");
            }
            Expr::Void
        }
        Expr::Let(name, value) => {
            context.insert(name, *value);
            Expr::Void
        }
        Expr::Constant(atom) => match atom {
            Atom::Name(name) => context.get(&name).unwrap().clone(),
            Atom::String(_) => todo!(),
        },

        _ => expr,
    }
}
