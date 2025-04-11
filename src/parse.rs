use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until},
    sequence::delimited,
};

#[allow(dead_code)]
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

