use crate::parse::*;
use std::collections::HashMap;

// HEADER: the main way the AST is traversed while
// keeping context
pub fn eval(expr: Expr, context: &mut HashMap<String, Expr>) -> Expr {
    // println!("Context: {:?}", context);
    match expr {
        Expr::Call(name, args) => {
            if name == "println" || name == "println!" {
                for arg in args {
                    let arg = eval(arg, context);
                    print!("{}", arg);
                }
                println!();
            } else {
                match context.get(&name).cloned() {
                    Some(Expr::Closure(parameters, body)) => {
                        let mut scope = context.clone();

                        for (parameter, arg) in parameters.iter().zip(args.into_iter()) {
                            let expr = eval(arg, &mut scope);
                            scope.insert(parameter.clone(), expr);
                        }
                        for expr in body {
                            if let Expr::Return(expr) = eval(expr.clone(), &mut scope) {
                                return *expr;
                            }
                            eval(expr.clone(), &mut scope);
                        }
                    }
                    _ => panic!("Function `{name} doesn't exist"),
                }
            }
            Expr::Void
        }
        Expr::Let(name, value) => {
            let expr = eval(*value, context);
            context.insert(name, expr);
            Expr::Void
        }
        Expr::Constant(ref atom) => match atom {
            Atom::Name(name) => {
                // not unwrapping and better error handling
                context.get(name).cloned().unwrap_or_else(|| {
                    eprintln!("Error: Variable '{}' not defined", name);
                    Expr::Void
                })
            }
            _ => expr,
        },
        Expr::Void | Expr::Closure(_, _) => expr,
        Expr::Function(name, args, body) => {
            context.insert(name, Expr::Closure(args, body));
            Expr::Void
        }
        Expr::Return(expr) => Expr::Return(Box::new(eval(*expr, context))),
    }
}
