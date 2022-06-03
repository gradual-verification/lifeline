use std::fmt;


#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum BinaryOp {
    LT,
    GT,
    GEQ,
    LEQ,
    EQ,
    NEQ,
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Assign
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
                BinaryOp::Or => "||",
                BinaryOp::NEQ => "!=",
                BinaryOp::Assign => "="
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type<'a>{
    Boolean,
    Integer,
    Ref(Box<Type<'a>>, usize),
    Struct(& 'a str)
}   

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match *&self {
                Type::Boolean => String::from("bool"),
                Type::Integer => String::from("int"),
                Type::Struct(n) => n.to_string(),
                Type::Ref(t, n) => format!("{} {}",t.to_string(), "*".repeat(*n)),
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
pub enum Expr<'a>{
    NumExpr(f32),
    BoolExpr(bool),
    Var(& 'a str), 
    UnaryExpr(UnaryOp, Box<Expr<'a>>),
    BinaryExpr(Box<Expr<'a>>, BinaryOp, Box<Expr<'a>>)
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match &*self {
                Expr::NumExpr(f) => f.to_string(),
                Expr::BoolExpr(b) => b.to_string(),
                Expr::Var(s) => s.to_string(),
                Expr::UnaryExpr(op, e) => format!("{}({})",op.to_string(),e.to_string()),
                Expr::BinaryExpr(l, op, r) => format!("{} {} {}", l.to_string(), op.to_string(), r.to_string())
            }
        )
    }
}



#[derive(Clone, Debug, PartialEq)]
pub enum Statement<'a>{
    ExprInst(Box<Expr<'a>>),
    Branch(Box<Expr<'a>>, Vec<Statement<'a>>,  Vec<Statement<'a>>),
    Decl(Box<Type<'a>>, &'a str, ),
    DeclAssign(Box<Type<'a>>,&'a str, Box<Expr<'a>>),
    Assign(&'a str, Box<Expr<'a>>),
    StructDefn(&'a str, Vec<(Type<'a>, & 'a str)>),
    ProcedureDefn(&'a str, Vec<(Type<'a>, & 'a str)>, Vec<Statement<'a>>, Type<'a>),
    Return(Box<Expr<'a>>)
}

/*
impl fmt::Display for Statement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match &*self {
                Statement::ExprInst(e) => e.to_string(),
                Statement::Branch(c, bt, bf) => format("if ({})")
                Statement::Decl(t, n) => 
                Statement::DeclAssign(t, n, v) => 
                Statement::Assign(n, v) => 
                Statement::StructDefn(n, flds) => 
                Statement::ProcedureDefn(n, params, body, ret) => 
                Statement::Return(e) => format!("return {};", e.to_string())
            }
        )
    }
}
*/

pub struct Block<'a> {
    pub contents: Vec<Statement<'a>>
}


#[derive(Clone, Debug, PartialEq)]
pub struct AST<'a> {
    pub procedures: Vec<Statement<'a>>,
    pub global_structs: Vec<Statement<'a>>
}

