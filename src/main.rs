use std::collections::HashMap;

mod eval;
mod parse;

#[allow(unused_variables)]
fn main() {
    let hello_input = include_str!("../hello_world.star");
    let closure_input = include_str!("../closure.star");
    let variables_input = include_str!("../input.star");
    let functions_input = include_str!("../input_functions.star");

    let (_, exprs) = parse::parser(closure_input).unwrap();
    let mut context = HashMap::new();
    for expr in exprs {
        // println!("Evaluating: {:?}", expr);
        let result = eval::eval(expr.clone(), &mut context);
        // println!("Result: {:?}", result);
    }
}
