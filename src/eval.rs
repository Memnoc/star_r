use crate::parse::*;

pub fn eval(expr: Expr) {
    match expr {
        Expr::Call(name, arg) => {
            if name == "println!" {
                println!("{arg}");
            }
        }
    }
}
