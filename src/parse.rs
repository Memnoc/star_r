use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, multispace0},
    combinator::map,
    error::ParseError,
    multi::{many0, separated_list0},
    sequence::{delimited, preceded},
};

//HEADER: Need WhiteSpace (ws) to handle white spaces
pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Atom {
    String(String),
    Name(String),
    Number(isize),
    Float(f64),
    Boolean(bool),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(string) => write!(f, "{string}"),
            Atom::Name(string) => write!(f, "{string}"),
            Atom::Number(number) => write!(f, "{number}"),
            Atom::Float(float) => write!(f, "{float}"),
            Atom::Boolean(bool) => write!(f, "{bool}"),
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Void,
    Constant(Atom),
    Let(String, Box<Expr>),
    Call(String, Vec<Expr>),
    Closure(Vec<String>, Vec<Expr>),
    Function(String, Vec<String>, Vec<Expr>),
    Return(Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Constant(atom) => write!(f, "{atom}"),
            _ => write!(f, ""),
        }
    }
}

// HEADER: for we need to be able to parse a constant
pub fn parse_constant(input: &str) -> IResult<&str, Expr> {
    parse_atom.map(Expr::Constant).parse(input)
}

// HEADER: for we need to be able to parse a return statement
// Pattern: return <expr>
pub fn parse_return(input: &str) -> IResult<&str, Expr> {
    let parser = preceded(tag("return"), ws(parse_expr));
    parser.map(|expr| Expr::Return(Box::new(expr))).parse(input)
}

// HEADER: parser for function calls with added delimiters
pub fn parse_call(input: &str) -> IResult<&str, Expr> {
    let parse_name = alpha1;
    let parse_arg = delimited(
        tag("("),
        separated_list0(tag(","), ws(parse_expr)),
        tag(")"),
    );
    let parser = (parse_name, parse_arg);
    parser
        .map(|(name, arg)| Expr::Call(name.to_string(), arg))
        .parse(input)
}

// HEADER: parser for variables with added prefixers
// Pattern: let <name> = <atom>
pub fn parse_variable(input: &str) -> IResult<&str, Expr> {
    let parse_name = preceded(tag("let"), ws(parse_identifier));
    let parse_equals = preceded(tag("="), ws(parse_expr));
    let parser = (parse_name, parse_equals);
    parser
        .map(|(name, value)| Expr::Let(name.to_string(), Box::new(value)))
        .parse(input)
}

// HEADER: parser for variables with added prefixers
// |name, argument| println(name);
// Pattern: |<arg>* | <expr>
pub fn parse_closure(input: &str) -> IResult<&str, Expr> {
    let parse_name = map(alpha1, String::from);
    let parse_args = delimited(
        tag("|"),
        separated_list0(tag(","), ws(parse_name)),
        tag("|"),
    );
    let parse_body = parse_expr;
    let parser = (ws(parse_args), ws(parse_body));

    parser
        .map(|(args, body)| Expr::Closure(args, vec![body]))
        .parse(input)
}

pub fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(alpha1, String::from).parse(input)
}

// HEADER: parser for function statements
// Parsing: fn <name>(<arg>*) { <expr*> }
pub fn parse_function(input: &str) -> IResult<&str, Expr> {
    let fn_keyword = ws(tag("fn"));
    let parse_fn_name = ws(parse_identifier);
    let parse_name = map(alpha1, String::from);
    let parse_args = delimited(
        tag("("),
        separated_list0(tag(","), ws(parse_name)),
        tag(")"),
    );
    let parse_body = delimited(tag("{"), many0(ws(parse_expr)), tag("}"));
    preceded(
        fn_keyword,
        ws(((parse_fn_name), ws(parse_args), ws(parse_body))),
    )
    .map(|(name, args, body)| Expr::Function(name, args, body))
    .parse(input)
}

// HEADER: it's easier and more functional to combine
// different parsers in sequence and search for a match
// the sequence actually has a sense (or reason) as I was
// having issues parsing keywords as variables
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_return,
        parse_function,
        parse_closure,
        parse_call,
        parse_variable,
        parse_constant,
    ))
    .parse(input)
}

// HEADER: Just commons sense at this point and
// a sequence matching with many0 for good measure
pub fn parser(input: &str) -> IResult<&str, Vec<Expr>> {
    many0(ws(parse_expr)).parse(input)
}
