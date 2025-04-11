use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until},
    character::streaming::alpha1,
    sequence::delimited,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Atom {
    String(String),
}

pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    delimited(tag("\""), take_until("\""), tag("\""))
        .map(|s: &str| {
            println!("Final string: {:?}", s);
            Atom::String(s.to_string())
        })
        .parse(input)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    Call(String, Atom),
}

pub fn parse_call(input: &str) -> IResult<&str, Expr> {
    let parse_name = alpha1;
    let parse_arg = delimited(tag("("), parse_string, tag(")"));
    let parser = (parse_name, parse_arg);
    parser
        .map(|(name, arg)| Expr::Call(name.to_string(), arg))
        .parse(input)
}
