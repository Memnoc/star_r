mod eval;
mod parse;

#[allow(unused_variables)]
fn main() {
    let input = include_str!("../input.star");
    let (_, expr) = parse::parse_call(input).unwrap();
    eval::eval(expr);
}
