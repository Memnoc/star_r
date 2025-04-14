use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::{complete::multispace0, streaming::alpha1},
    error::ParseError,
    multi::many0,
    sequence::{delimited, preceded},
};

//HEADER: Need WhiteSpace (ws) ignores
// the white spaces around statements and expressions
// and variables declarations
pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

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
    Let(String, Atom),
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

// HEADER: parser for function calls
// let <name> = <atom>
pub fn parse_variable(input: &str) -> IResult<&str, Expr> {
    let parse_name = preceded(tag("let"), ws(alpha1));
    let parse_equals = preceded(tag("="), ws(parse_string));
    let parser = (parse_name, parse_equals);
    parser
        .map(|(name, value)| Expr::Let(name.to_string(), value))
        .parse(input)
}

// HEADER: for we need to be able to combine variable
// declarations and assignments to function calls
pub fn parse_expr(input: &str) -> IResult<&str, Vec<Expr>> {
    many0(alt((parse_variable, parse_call))).parse(input)
}
