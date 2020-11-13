use crate::source::Source;
use crate::to_lowercase_first_char;
use crate::token::Location;

use std::rc::Rc;

#[derive(Debug)]
pub struct ASTs {
    pub source: Rc<Source>,
    pub class: Class,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub loc: Location,
    pub name: String,
    pub vars: Vec<ClassVarDec>,
    pub subroutines: Vec<SubroutineDec>,
}

impl Class {
    pub fn static_vars(&self) -> Vec<&ClassVarDec> {
        self.vars
            .iter()
            .filter(|var| var.modifier == ClassVarModifier::Static)
            .collect()
    }

    pub fn field_vars(&self) -> Vec<&ClassVarDec> {
        self.vars
            .iter()
            .filter(|var| var.modifier == ClassVarModifier::Field)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ClassVarDec {
    pub loc: Location,
    pub modifier: ClassVarModifier,
    pub typ: Type,
    pub names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassVarModifier {
    Static,
    Field,
}

impl ClassVarModifier {
    pub fn display(&self) -> String {
        to_lowercase_first_char(format!("{:?}", self).as_str())
    }
}

#[derive(Debug, Clone)]
pub struct SubroutineDec {
    pub loc: Location,
    pub modifier: SubroutineModifier,
    pub typ: ReturnType,
    pub name: String,
    pub parameters: Vec<ParameterDec>,
    pub body: SubroutineBody,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubroutineModifier {
    Constructor,
    Function,
    Method,
}

impl SubroutineModifier {
    pub fn display(&self) -> String {
        to_lowercase_first_char(format!("{:?}", self).as_str())
    }
}

#[derive(Debug, Clone)]
pub struct ParameterDec {
    pub loc: Location,
    pub typ: Type,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct SubroutineBody {
    pub vars: Vec<VarDec>,
    pub statements: Statements,
}

#[derive(Debug, Clone)]
pub struct VarDec {
    pub loc: Location,
    pub typ: Type,
    pub names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Statements {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    If(IfStatement),
    While(WhileStatement),
    Do(DoStatement),
    Return(ReturnStatement),
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub loc: Location,
    pub name: String,
    pub accessor: Option<Expr>,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub loc: Location,
    pub cond: Expr,
    pub statements: Statements,
    pub else_branch: Option<Statements>,
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub loc: Location,
    pub cond: Expr,
    pub statements: Statements,
}

#[derive(Debug, Clone)]
pub struct DoStatement {
    pub loc: Location,
    pub call: SubroutineCall,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub loc: Location,
    pub expr: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub loc: Location,
    pub lhs: Box<Term>,
    pub rhs: Box<Option<(Op, Expr)>>,
}

#[derive(Debug, Clone)]
pub enum Term {
    Integer(u16),
    Str(String),
    Keyword(KeywordConst),
    Var(String),
    IndexAccess(String, Expr),
    Call(SubroutineCall),
    Expr(Expr),
    Unary(UnaryOp, Box<Term>),
}

#[derive(Debug, Clone)]
pub enum KeywordConst {
    True,
    False,
    Null,
    This,
}

impl KeywordConst {
    pub fn display(&self) -> String {
        to_lowercase_first_char(format!("{:?}", self).as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Lt,
    Gt,
    Eq,
}

impl Op {
    pub fn parse(sym: char) -> Option<Op> {
        match sym {
            '+' => Some(Op::Add),
            '-' => Some(Op::Sub),
            '*' => Some(Op::Mul),
            '/' => Some(Op::Div),
            '&' => Some(Op::And),
            '|' => Some(Op::Or),
            '<' => Some(Op::Lt),
            '>' => Some(Op::Gt),
            '=' => Some(Op::Eq),
            _ => None,
        }
    }

    pub fn display(&self) -> String {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::And => "&amp;",
            Op::Or => "|",
            Op::Lt => "&lt;",
            Op::Gt => "&gt;",
            Op::Eq => "=",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus,
    Not,
}

impl UnaryOp {
    pub fn parse(sym: char) -> Option<UnaryOp> {
        match sym {
            '-' => Some(UnaryOp::Minus),
            '~' => Some(UnaryOp::Not),
            _ => None,
        }
    }

    pub fn display(&self) -> String {
        match self {
            UnaryOp::Minus => "-",
            UnaryOp::Not => "~",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct SubroutineCall {
    pub loc: Location,
    pub reciever: Option<String>,
    pub name: String,
    pub exprs: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Char,
    Boolean,
    Class(String),
}

impl Type {
    pub fn display(&self) -> String {
        match self {
            Type::Int | Type::Char | Type::Boolean => {
                to_lowercase_first_char(format!("{:?}", self).as_str())
            }
            Type::Class(name) => name.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReturnType {
    Void,
    Type(Type),
}
