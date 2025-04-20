use std::collections::HashMap;

mod eval;
mod parse;

#[allow(unused_variables)]
fn main() {
    let input = include_str!("../input.star");
    let (_, exprs) = parse::parser(input).unwrap();
    let mut context = HashMap::new();
    for expr in exprs {
        let result = eval::eval(expr, &mut context);
        println!("Result: {:?}", result);
        println!("Result: {:?}", result);
    }
}
