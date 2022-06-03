use super::ast::BinaryOp;
use super::ast::Expr;
use super::ast::Expr::{NumExpr, BoolExpr, Var, BinaryExpr};
use nom::IResult;
use nom::character::complete::char;
use nom::bytes::complete::is_not;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::number::complete::{float};
use nom::combinator::*;
use nom::multi::*;
use nom::branch::*;
use nom::sequence::tuple;
use nom::sequence::{pair, delimited};
use nom::bytes::complete::{tag};

pub fn expr(input: &str) -> IResult<&str, Expr> {
    and(input)
}

fn parens(input: &str) -> IResult<&str, Expr> {
    delimited(ig(tag("(")), term, ig(tag(")")))(input)
}

fn and(input: &str) -> IResult<&str, Expr> {
    let (input, lhs) = or(input)?;
    let (input, rem) = many0(pair(ig(tag("&&")), term))(input)?;
    Ok((input, merge_binary(lhs, rem)))
}

fn or(input: &str) -> IResult<&str, Expr> {
    let (input, lhs) = compare(input)?;
    let (input, rem) = many0(pair(ig(tag("||")), term))(input)?;
    Ok((input, merge_binary(lhs, rem)))
}

fn compare(input: &str) -> IResult<&str, Expr> {
    let (input, lhs) = factor(input)?;
    let (input, rem) = many0(pair(ig(alt((tag("<"), tag(">"), tag("<="), tag(">="), tag("!=")))), term))(input)?;
    Ok((input, merge_binary(lhs, rem)))
}

fn factor(input: &str) -> IResult<&str, Expr> {
    let (input, lhs) = term(input)?;
    let (input, rem) = many0(pair(ig(alt((tag("*"), tag("/")))), term))(input)?;
    Ok((input, merge_binary(lhs, rem)))
}
fn term(input: &str) -> IResult<&str, Expr> {
    let (input, lhs) = atom(input)?;
    let (input, rem) = many0(pair(ig(alt((tag("+"), tag("-")))), atom))(input)?;
    Ok((input, merge_binary(lhs, rem)))
}

fn merge_binary<'a>(lhs: Expr<'a>, rhs_list: Vec<(&str,Expr<'a>)>) ->Expr<'a> {

    let bop = |s:&str| {
        match s{
        "+" => BinaryOp::Add,
        "-" => BinaryOp::Subtract,
        "*" => BinaryOp::Multiply,
        "/" => BinaryOp::Divide,
        "<" => BinaryOp::LT,
        ">" => BinaryOp::GT,
        "<=" => BinaryOp::LEQ,
        ">=" => BinaryOp::GEQ,
        "==" => BinaryOp::EQ,
        "!=" => BinaryOp::NEQ,
        "&&" => BinaryOp::And,
        "||" => BinaryOp::Or,
        _ => todo!()
        }
    };
    rhs_list.into_iter().fold(lhs, |curr, item| BinaryExpr(Box::new(curr), bop(item.0), Box::new(item.1)))
}

fn atom(input: &str) -> IResult<&str, Expr> {
   ig(alt((var, boolean, numerical, parens)))(input)
}

fn var(input: &str) -> IResult<&str, Expr> {
    let (i, r) = identifier(input)?;
    Ok((i, Var(r)))
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
          alt((alpha1, tag("_"))),
          many0(alt((alphanumeric1, tag("_"))))
        )
      )(input)
}

fn boolean(input: &str) -> IResult<&str, Expr> {
    return alt((
        value(BoolExpr(true), tag("true")), 
        value(BoolExpr(false), tag("false"))
    ))(input);
}
fn numerical(input: &str) -> IResult<&str, Expr> {
    let (i, r):(&str, f32) = float(input)?;
    Ok((i, NumExpr(r)))
}


pub fn ig<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    ignore,
    inner,
    ignore
  )
}

fn ignore<'a, E: ParseError<&'a str>>(input:& 'a str) -> IResult<&'a str, (), E> {
    value((), many0(alt((pinline_comment, peol_comment, value((), multispace0)))))(input)
}

fn pinline_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
      (), // Output is thrown away.
      tuple((
        tag("(*"),
        take_until("*)"),
        tag("*)")
      ))
    )(i)
  }

fn peol_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E>
  {
    value(
      (), // Output is thrown away.
      pair(char('%'), is_not("\n\r"))
    )(i)
  }