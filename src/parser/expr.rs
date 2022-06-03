use super::ast::BinaryOp;
use super::ast::Expr;
use super::ast::Expr::{NumExpr, BoolExpr, Var, BinaryExpr, Call, UnaryExpr};
use super::ast::UnaryOp;
use nom::IResult;
use nom::Parser;
use nom::character::complete::char;
use nom::bytes::complete::is_not;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace1;
use nom::error::ParseError;
use nom::number::complete::{float};
use nom::combinator::*;
use nom::multi::*;
use nom::Err::Error;
use nom::branch::*;
use nom::sequence::tuple;
use nom::sequence::{pair, delimited};
use nom::bytes::complete::{tag};


pub fn expr(input: &str) -> IResult<&str, Expr> {
    assign(input)
}

fn parens(input: &str) -> IResult<&str, Expr> {
    delimited(ig(tag("(")), term, ig(tag(")")))(input)
}

fn assign(input: &str) -> IResult<&str, Expr> {
  let (input, lhs) = and(input)?;
  let (input, rem) = many0(pair(ig(tag("=")), term))(input)?;
  Ok((input, merge_binary(lhs, rem)))
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
    let (input, rem) = many0(pair(ig(alt((tag("<="), tag(">="), tag("<"), tag(">"), tag("!="), tag("==")))), term))(input)?;
    Ok((input, merge_binary(lhs, rem)))
}

fn factor(input: &str) -> IResult<&str, Expr> {
    let (input, lhs) = term(input)?;
    let (input, rem) = many0(pair(ig(alt((tag("*"), tag("/")))), term))(input)?;
    Ok((input, merge_binary(lhs, rem)))
}
fn term(input: &str) -> IResult<&str, Expr> {
    let (input, lhs) = atom(input)?;
    let (input, rem) = many0(pair(ig(alt((tag("+"), tag("-")))), unary))(input)?;
    Ok((input, merge_binary(lhs, rem)))
}

pub fn value_closure<I, O1, O2, E: ParseError<I>, F1, F2>(
  mut parser: F1,
  mut val: F2
) -> impl FnMut(I) -> IResult<I, O1, E>
where
  F1: Parser<I, O2, E>,
  F2: FnMut(O2) -> O1
{

  move |input: I| parser.parse(input).map(|(i, r)| (i, val(r)))
}


fn unary(input: &str) -> IResult<&str, Expr> {

    let (input, result) = pair(
      opt(
        alt((
          map_res(
            many1_count(tag("!")),
            |u:usize| -> Result<UnaryOp, nom::Err<&str>>{
              if u % 2 == 1 {
                Ok(UnaryOp::Not)
              } else {
                Err(Error(""))
              } 
            }
          ), 
          value(UnaryOp::AddressOf, tag("&")), 
          map_res(
            many1_count(tag("*")),
            |u:usize| -> Result<UnaryOp, nom::Err<&str>> {
                Ok(UnaryOp::Deref(u))
            }
          )
        ))
      ),
    atom)(input)?;
    match result.0 {
      Some(op) => {
        Ok((input, UnaryExpr(op, Box::new(result.1))))
      } 
      None => {
        Ok((input, result.1))
      }
    }
}

fn merge_binary<'a>(lhs: Expr<'a>, rhs_list: Vec<(&str,Expr<'a>)>) ->Expr<'a> {

    let bop = |s:&str| {
        match s{
        "+" => BinaryOp::Add,
        "-" => BinaryOp::Subtract,
        "*" => BinaryOp::Multiply,
        "/" => BinaryOp::Divide,
        "<=" => BinaryOp::LEQ,
        ">=" => BinaryOp::GEQ,
        "<" => BinaryOp::LT,
        ">" => BinaryOp::GT,
        "==" => BinaryOp::EQ,
        "!=" => BinaryOp::NEQ,
        "&&" => BinaryOp::And,
        "||" => BinaryOp::Or,
        "=" => BinaryOp::Assign,
        _ => todo!()
        }
    };
    rhs_list.into_iter().fold(lhs, |curr, item| BinaryExpr(Box::new(curr), bop(item.0), Box::new(item.1)))
}

fn atom(input: &str) -> IResult<&str, Expr> {
   ig(alt((call, var, numerical, parens, )))(input)
}

fn var(input: &str) -> IResult<&str, Expr> {
  let (input, id) = identifier(input)?;
  match id {
    "true" => Ok((input, BoolExpr(true))),
    "false" => Ok((input, BoolExpr(false))),
    _ => {
      let (input, params) = opt(ig(delimited(char('('), ig(separated_list0(ig(tag(",")), expr)), char(')'))))(input)?;
      match params {
        Some(p) => {
          Ok((input, Call(id, p)))
        } 
        None => {
          Ok((input, Var(id)))
        }
      }
    }
  }
}
fn call(input: &str) -> IResult<&str, Expr> {
  let (input, id) = identifier(input)?;
  let (input, params) = ig(delimited(char('('), ig(separated_list0(ig(tag(",")), expr)), char(')')))(input)?;
  Ok((input, Call(id, params)))
} 

pub fn identifier(input: &str) -> IResult<&str, &str> {
  recognize(
      pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_"))))
      )
    )(input)
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
    value((), many0(alt((pinline_comment, peol_comment, value((), multispace1)))))(input)
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

#[cfg(test)]
mod tests {
  use crate::parser::{Expr::{BinaryExpr, NumExpr, BoolExpr, Var, Call}, ast::BinaryOp, expr};

    #[test]
    fn binary_add() {
        let parsed_add = expr("2 + 2");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(NumExpr(2.0)), BinaryOp::Add, Box::new(NumExpr(2.0)))))
        );
    }
    #[test]
    fn binary_subt() {
        let parsed_add = expr("2 - 2");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(NumExpr(2.0)), BinaryOp::Subtract, Box::new(NumExpr(2.0)))))
        );
    }

    #[test]
    fn binary_mult() {
        let parsed_add = expr("2 * 2");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(NumExpr(2.0)), BinaryOp::Multiply, Box::new(NumExpr(2.0)))))
        );
    }
    #[test]
    fn binary_div() {
        let parsed_add = expr("2 / 2");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(NumExpr(2.0)), BinaryOp::Divide, Box::new(NumExpr(2.0)))))
        );
    }

    #[test]
    fn binary_and() {
        let parsed_add = expr("true && false");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(BoolExpr(true)), BinaryOp::And, Box::new(BoolExpr(false)))))
        );
    }

    #[test]
    fn binary_or() {
        let parsed_add = expr("true || false");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(BoolExpr(true)), BinaryOp::Or, Box::new(BoolExpr(false)))))
        );
    }
    #[test]
    fn binary_lt() {
        let parsed_add = expr("2 < 2");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(NumExpr(2.0)), BinaryOp::LT, Box::new(NumExpr(2.0)))))
        );
    }
    #[test]
    fn binary_gt() {
        let parsed_add = expr("2 > 2");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(NumExpr(2.0)), BinaryOp::GT, Box::new(NumExpr(2.0)))))
        );
    }
    #[test]
    fn binary_leq() {
        let parsed_add = expr("2 <= 2");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(NumExpr(2.0)), BinaryOp::LEQ, Box::new(NumExpr(2.0)))))
        );
    }

    #[test]
    fn binary_geq() {
        let parsed_add = expr("2 >= 2");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(NumExpr(2.0)), BinaryOp::GEQ, Box::new(NumExpr(2.0)))))
        );
    }

    #[test]
    fn binary_neq() {
        let parsed_add = expr("2 != 2");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(NumExpr(2.0)), BinaryOp::NEQ, Box::new(NumExpr(2.0)))))
        );
    }
    #[test]
    fn binary_eq() {
        let parsed_add = expr("2 == 2");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(NumExpr(2.0)), BinaryOp::EQ, Box::new(NumExpr(2.0)))))
        );
    }

    #[test]
    fn binary_assign() {
        let parsed_add = expr("i = 2");
        assert_eq!(
          parsed_add,
          Ok(("", BinaryExpr(Box::new(Var("i")), BinaryOp::Assign, Box::new(NumExpr(2.0)))))
        );
    }

    #[test]
    fn atom_boolean_true() {
      let parsed_add = expr("true");
      assert_eq!(
        parsed_add,
        Ok(("", BoolExpr(true)))
      );
    }
    #[test]
    fn atom_boolean_false() {
      let parsed_add = expr("false");
      assert_eq!(
        parsed_add,
        Ok(("", BoolExpr(false)))
      );
    }

    #[test]
    fn var() {
      let parsed_add = expr("truef");
      assert_eq!(
        parsed_add,
        Ok(("", Var("truef")))
      );
    }

    #[test]
    fn call() {
      let parsed_add = expr("truef()");
      assert_eq!(
        parsed_add,
        Ok(("", Call("truef", vec![])))
      );
    }


}