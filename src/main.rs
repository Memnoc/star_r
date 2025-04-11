use parse::parse_call;

mod parse;

#[allow(unused_variables)]
fn main() {
    let input = include_str!("../input.star");
    let string = "println(\"hello, world!\")";
    let _ = parse_call(string);
}
