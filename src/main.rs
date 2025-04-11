use parse::parse_string;

mod parse;

#[allow(unused_variables)]
fn main() {
    let input = include_str!("../input.star");
    let string = "\"Hello, world!\"";
    parse_string(string);
}
