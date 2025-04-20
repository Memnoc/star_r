use crate::parse::*;
use std::collections::HashMap;

// HEADER: the main way the AST is traversed while
// keeping context
pub fn eval(expr: Expr, context: &mut HashMap<String, Expr>) -> Expr {
    // println!("Context: {:?}", context);
    match expr {
        Expr::Call(name, args) => {
            if name == "println" {
                for arg in args {
                    let arg = eval(arg, context);
                    print!("{arg:?}");
                }
            } else {
                match context.get(&name) {
                    Some(Expr::Closure(parameters, body)) => {
                        let mut scope = context.clone();

                        for (parameter, arg) in parameters.iter().zip(args.into_iter()) {
                            let expr = eval(arg, &mut scope);
                            scope.insert(parameter.clone(), expr);
                        }
                        for expr in body {
                            eval(expr.clone(), &mut scope);
                        }
                    }
                    _ => panic!("Function `{name} doesn't exist"),
                }
            }
            Expr::Void
        }
        Expr::Let(name, value) => {
            context.insert(name, *value);
            Expr::Void
        }
        Expr::Constant(ref atom) => match atom {
            Atom::Name(name) => {
                // println!("Looking up variable:{}", name);
                context.get(name).cloned().unwrap_or_else(|| {
                    eprintln!("Error: Variable '{}' not defined", name);
                    Expr::Void
                })
            }
            _ => expr,
        },
        Expr::Void | Expr::Closure(_, _) => expr,
    }
}
