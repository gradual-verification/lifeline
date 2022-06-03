use std::{fmt};


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
    Deref(usize),
    AddressOf,
    Not
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match *self {
                UnaryOp::Deref(_) => "*",
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
    Call(& 'a str, Vec<Expr<'a>>),
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
                Expr::BinaryExpr(l, op, r) => format!("{} {} {}", l.to_string(), op.to_string(), r.to_string()),
                Expr::Call(nm, params) => {
                    let stringified:Vec<String> = params.into_iter().map(|p| {
                        p.to_string()
                    }).collect();
                    format!("{}({})", nm, stringified.join(","))
                }
            }
        )
    }
}



#[derive(Clone, Debug, PartialEq)]
pub enum Statement<'a>{
    ExprStmt(Box<Expr<'a>>),
    IfStmt(Box<Expr<'a>>, Vec<Statement<'a>>,  Vec<Statement<'a>>),
    Decl(Box<Type<'a>>, &'a str, ),
    DeclAssign(Box<Type<'a>>,&'a str, Box<Expr<'a>>),
    StructDefn(&'a str, Vec<(Type<'a>, & 'a str)>),
    ProcedureDefn(&'a str, Vec<(Type<'a>, & 'a str)>, Vec<Statement<'a>>, Type<'a>),
    Return(Box<Expr<'a>>)
}

pub fn print_statements<'a>(stmts:&Vec<Statement>, indentation: usize) -> String {
    stmts.into_iter().map(|s| {
        print_statement(&s, indentation)
    }).fold("  ".repeat(indentation), |acc, p| {
        format!("{}{}\n", acc, p)
    })
}

fn print_params(params: &Vec<(Type, &str)>) -> String {
    let stringified:Vec<String> = params.into_iter().map(|p| {
        format!("{} {}", p.0.to_string(), p.1.to_string())
    }).collect();
    stringified.join(",")
}

pub fn print_statement<'a>(stmt: &Statement<'a>, indentation: usize) -> String{
    let indent = "  ".repeat(indentation);
    let formatted = match &*stmt {
        Statement::ExprStmt(eb) => format!("{};",eb.to_string()),
        Statement::IfStmt(cond, tb, fb)  => {
            let closing = if fb.len() > 0 {
                if fb.len() == 1 && matches!(*fb.first().unwrap(), Statement::IfStmt(_, _, _)) {
                    format!("else {}", print_statement(fb.first().unwrap(), 0))
                }else{
                    format!("else {{{}{}}}", print_statements(fb, indentation + 1), indent)
                }
            }else{
                String::from("")
            };
            format!("if ({}) {{{}{}}} {}", cond.to_string(), indent, print_statements(tb, indentation + 1), closing)
        }
        Statement::Decl(t, n) => format!("{} {};", t.to_string(), n),
        Statement::DeclAssign(t, n, e) => format!("{} {} = {};", t.to_string(), n, e.to_string()), 
        Statement::ProcedureDefn(nm, params, body, ret) => {
            format!("fn {}({}) -> {} {{{}{}}}", nm, print_params(params), ret.to_string(), print_statements(body, indentation+1), indent)
        }
        Statement::StructDefn(nm, flds) => {
            let stringified:Vec<String> = flds.into_iter().map(|p| {
                format!("{} {}", p.0.to_string(), p.1.to_string())
            }).collect();
            format!("struct {} {{{}\n{}}}", nm, stringified.join(",\n"), indent)
        }
        Statement::Return(e) => format!("return {};", e.to_string())
    };
    format!("{}{}", indent, formatted)
}


#[derive(Clone, Debug, PartialEq)]
pub struct AST<'a> {
    pub procedures: Vec<Statement<'a>>,
    pub global_structs: Vec<Statement<'a>>
}

impl fmt::Display for AST<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            format!("{}\n{}", print_statements(&self.global_structs, 0), print_statements(&self.procedures, 0))
        )
    }
}