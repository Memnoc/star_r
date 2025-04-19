use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::{complete::alpha1, complete::multispace0},
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
#[derive(Debug, Clone)]
pub enum Atom {
    String(String),
    Name(String),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(string) => write!(f, "{string}"),
            Atom::Name(string) => write!(f, "{string}"),
        }
    }
}

// HEADER: parser for strings
pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    delimited(tag("\""), take_until("\""), tag("\""))
        .map(|s: &str| Atom::String(s.to_string()))
        .parse(input)
}

// HEADER: parser for variables
pub fn parse_name(input: &str) -> IResult<&str, Atom> {
    let parse_name = alpha1;
    let parser = parse_name;
    parser
        .map(|name: &str| Atom::Name(name.to_string()))
        .parse(input)
}

pub fn parse_atom(input: &str) -> IResult<&str, Atom> {
    alt((parse_name, parse_string)).parse(input)
}

// NOTE: Struct for Functions
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Void,
    Constant(Atom),
    Let(String, Box<Expr>),
    Call(String, Box<Expr>),
}

// HEADER: for we need to be able to parse a constant
pub fn parse_constant(input: &str) -> IResult<&str, Expr> {
    parse_atom.map(Expr::Constant).parse(input)
}

// HEADER: parser for function calls
pub fn parse_call(input: &str) -> IResult<&str, Expr> {
    let parse_name = alpha1;
    let parse_arg = delimited(tag("!("), parse_expr, tag(")"));
    let parser = (parse_name, parse_arg);
    parser
        .map(|(name, arg)| Expr::Call(name.to_string(), Box::new(arg)))
        .parse(input)
}

// HEADER: parser for function calls
// let <name> = <atom>
pub fn parse_variable(input: &str) -> IResult<&str, Expr> {
    let parse_name = preceded(tag("let"), ws(alpha1));
    let parse_equals = preceded(tag("="), ws(parse_atom));
    let parser = (parse_name, parse_equals);
    parser
        .map(|(name, value)| Expr::Let(name.to_string(), Box::new(Expr::Constant(value))))
        .parse(input)
}

// HEADER: for we need to be able to combine variable
// declarations and assignments to function calls
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_variable, parse_call, parse_constant)).parse(input)
}

pub fn parser(input: &str) -> IResult<&str, Vec<Expr>> {
    many0(ws(parse_expr)).parse(input)
}
