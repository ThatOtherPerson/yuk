pub use super::interpret::{Value};

pub type Block = Vec<Statement>;

pub type ExpressionList = Vec<Expression>;

#[derive(Debug, Clone)]
pub struct InnerBlock {
    pub block: Vec<Statement>,
    pub return_exp: Option<Box<Expression>>
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Declaration(Declaration),
    Throw(Expression),
    If(Expression, InnerBlock, Option<InnerBlock>),
    Empty
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Variable(Identifier, Option<Expression>),
    Function(Identifier, Function)
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Positive,
    Negative,

    LogicalNot
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,

    Multiply,
    Divide,

    LogicalAnd,
    LogicalOr,

    StrictEquals
}

#[derive(Debug, Clone)]
pub enum Expression {
    Assignment(Access, Box<Expression>),
    Access(Access),
    Call(Box<Expression>, ExpressionList),
    New(Box<Expression>, ExpressionList),
    Literal(Value),
    Function(Function),
    Unary(UnaryOp, Box<Expression>),
    Binary(BinaryOp, Box<Expression>, Box<Expression>),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    Object(Vec<(String, Expression)>),
    This
}

#[derive(Debug, Clone)]
pub enum Access {
    Member(Box<Expression>, Accessor),
    Identifier(Identifier)
}

#[derive(Debug, Clone)]
pub enum Accessor {
    Identifier(Identifier),
    Expression(Box<Expression>)
}

#[derive(Debug, Clone)]
pub struct Function {
    pub id: Option<Identifier>,
    pub parameters: Vec<String>,
    pub body: InnerBlock,
    pub source: String
}

pub type Identifier = String;
