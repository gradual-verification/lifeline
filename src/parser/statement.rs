
use nom::IResult;
use nom::bytes::complete::{tag};
use nom::combinator::*;
use nom::branch::*;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use super::ast::{AST, Statement, Type};
use super::ast::Statement::{IfStmt, StructDefn, ProcedureDefn};
use super::expr;
use super::expr::{identifier, ig};
use nom::character::complete::char;

pub fn ast(input: &str) -> IResult<&str, AST> {
    let (input, statements) = many0(alt((struct_decl, proc_defn)))(input)?;
    let mut structs = vec![];
    let mut procedures = vec![];
    statements.into_iter().for_each(|s| {
        match s {
            StructDefn(_, _) => {
                structs.push(s)
            }
            ProcedureDefn(_, _, _, _) => {
                procedures.push(s)
            }
            _ => {
            }
        }
    });
    Ok((input, AST {
        procedures: procedures,
        global_structs: structs
    }))
}

pub fn struct_decl(input: &str) -> IResult<&str, Statement>{
    let field = terminated(pair(type_name, identifier),ig(char(';')));
    let (input, result) = ig(tuple((identifier, delimited(ig(char('{')), many0(ig(field)), ig(char('}'))))))(input)?;
    Ok((input, StructDefn(result.0, result.1)))
}

pub fn proc_defn(input: &str) -> IResult<&str, Statement>{
    let param_list = delimited(ig(char('(')), separated_list0(char(','), pair(type_name, identifier)), ig(char(',')));
    let return_type = preceded(ig(tag("->")), type_name);
    let (input, func) = tuple((preceded(ig(tag("fn")), identifier), param_list, return_type, block))(input)?;
    Ok((input, ProcedureDefn(func.0,func.1,func.3, func.2)))
}


pub fn statement(input: &str) -> IResult<&str, Statement>{
    ig(alt((expr_stmt, decl, decl_assign, ret, if_stmt)))(input)
}

pub fn block(input: &str) -> IResult<&str, Vec<Statement>> {
    ig(delimited(ig(char('{')), many0(statement), ig(char('}'))))(input)
}

fn if_stmt(input: &str) -> IResult<&str, Statement> {
    let cond = ig(delimited(char('('), expr, char(')')));
    let (input, branches) = tuple(
        (
            preceded(ig(tag("if")),pair(cond, block)),
             opt(preceded(ig(tag("else")), alt((if_stmt_block, block)))))
    )(input)?;
    let resulting_expr = match branches.1 {
        Some(f_branch) => {
            IfStmt(Box::new(branches.0.0), branches.0.1, f_branch)
        }
        None => {
            IfStmt(Box::new(branches.0.0), branches.0.1, vec![])
        }
    };
    Ok((input, resulting_expr))
}


fn if_stmt_block(input: &str) -> IResult<&str, Vec<Statement>>{
    let (input, inner_if) = if_stmt(input)?;
    Ok((input, vec![inner_if]))
}

fn decl_assign(input: &str) -> IResult<&str, Statement> {
    let (input, result) = tuple((type_name, identifier, tag("="), expr))(input)?;
    Ok((input, Statement::DeclAssign(Box::new(result.0), result.1, Box::new(result.3))))
}   

fn decl(input: &str) -> IResult<&str, Statement> {
    let (input, result) = tuple((type_name, ig(identifier)))(input)?;
    Ok((input, Statement::Decl(Box::new(result.0), result.1)))
}

fn expr_stmt(input: &str) -> IResult<&str, Statement> {
    let (input, result) = expr(input)?;
    Ok((input, Statement::ExprStmt(Box::new(result))))
}

fn ret(input: &str) -> IResult<&str, Statement> {
    let (input, result) = pair(ig(tag("return")), expr)(input)?;
    Ok((input, Statement::Return(Box::new(result.1))))
}

pub fn type_name(input: &str) -> IResult<&str, Type> {
    let (input, r) = pair(ig(alt((identifier, tag("bool"), tag("int")))), many0(ig(tag("*"))))(input)?;
    let primary = match r.0 {
        "bool" => Type::Boolean,
        "int" => Type::Integer,
        _ => Type::Struct(r.0),
    };
    if r.1.is_empty() {
        Ok((input, primary))
    }else{
        Ok((input, Type::Ref(Box::new(primary), r.1.len())))
    }
}
