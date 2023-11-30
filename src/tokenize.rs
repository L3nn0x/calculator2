use super::token::{Token, Operator};

use nom::{
    character::complete::{char, i64, hex_digit1, multispace0},
    number::complete::*,
    combinator::{map, map_res}, IResult, sequence::{tuple, delimited}, branch::alt, bytes::complete::tag, Parser, error::ParseError, multi::many0
};

fn ws<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    f: F,
  ) -> impl Parser<&'a str, O, E> {
    delimited(multispace0, f, multispace0)
}

fn integer(i: &str) -> IResult<&str, Token> {
    map(i64, |res| Token::Integer(res))(i)
}

fn hexa(i: &str) -> IResult<&str, Token> {
    map_res(tuple((
        alt((tag("0x"), tag("0X"))),
        hex_digit1
    )), |(_, digits)| i64::from_str_radix(digits, 16).map(|n| Token::Integer(n)))(i)
}

fn float(i: &str) -> IResult<&str, Token> {
    map(double, |res| Token::Float(res))(i)
}

fn number(i: &str) -> IResult<&str, Token> {
    alt((
        ws(hexa),
        ws(float),
        ws(integer),
    ))(i)
}

fn lparent(i: &str) -> IResult<&str, Token> {
    map(char('('), |_| Token::LeftParenthesis)(i)
}

fn rparent(i: &str) -> IResult<&str, Token> {
    map(char(')'), |_| Token::RightParenthesis)(i)
}

fn operators(i: &str) -> IResult<&str, Token> {
    map(alt((char('+'), char('-'), char('/'), char('*'), char('^'))), |t| Token::Operator(Operator::from(t).unwrap()))(i)
}

fn expr(i: &str) -> IResult<&str, Vec<Token>> {
    many0(alt((
        ws(lparent),
        ws(rparent),
        ws(operators),
        number)))(i)
}

pub(crate) fn tokenize(s: String) -> Result<Vec<Token>, String> {
    expr(&s).map_err(|e| format!("{:?}", e)).and_then(|(_, r)| Ok(r))
}