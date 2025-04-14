mod eval;
mod parse;

#[allow(unused_variables)]
fn main() {
    let input = include_str!("../input.star");
    let (rest, expr) = parse::parse_expr(input).unwrap();
    dbg!(&rest, &expr);
    // eval::eval(expr);
}
