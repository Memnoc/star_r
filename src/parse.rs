use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until},
    character::streaming::alpha1,
    sequence::delimited,
};

// NOTE: Struct for Strings
#[allow(dead_code)]
#[derive(Debug)]
pub enum Atom {
    String(String),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(string) => write!(f, "{string}"),
        }
    }
}

// HEADER: parser for strings
pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    delimited(tag("\""), take_until("\""), tag("\""))
        .map(|s: &str| Atom::String(s.to_string()))
        .parse(input)
}

// NOTE: Struct for Functions
#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    Call(String, Atom),
}

// HEADER: parser for function calls
pub fn parse_call(input: &str) -> IResult<&str, Expr> {
    let parse_name = alpha1;
    let parse_arg = delimited(tag("!("), parse_string, tag(")"));
    let parser = (parse_name, parse_arg);
    parser
        .map(|(name, arg)| Expr::Call(format!("{}!", name), arg))
        .parse(input)
}
