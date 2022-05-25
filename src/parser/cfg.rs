use std::fmt;


#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum BinaryOp {
    LT,
    GT,
    GEQ,
    LEQ,
    EQ,
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or
}
impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match *self {
                BinaryOp::LT => "<",
                BinaryOp::GT => ">",
                BinaryOp::GEQ => ">=",
                BinaryOp::LEQ => "<=",
                BinaryOp::EQ => "==",
                BinaryOp::Add => "+",
                BinaryOp::Subtract => "-",
                BinaryOp::Multiply => "*",
                BinaryOp::Divide => "/",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||"
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum UnaryOp {
    Deref,
    AddressOf,
    Not
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match *self {
                UnaryOp::Deref => "*",
                UnaryOp::AddressOf => "&",
                UnaryOp::Not => "!"
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr{
    NumExpr(f32),
    UnaryExpr(UnaryOp, Box<Expr>),
    BinaryExpr(Box<Expr>, BinaryOp, Box<Expr>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement{
    ExprStmt(Expr),
    Assign(Expr,Expr)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProcedureDefn {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructDefn {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CFG {
    pub procedures: Vec<ProcedureDefn>,
    pub structs: Vec<StructDefn>
}