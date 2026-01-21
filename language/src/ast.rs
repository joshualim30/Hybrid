// ast.rs
// Abstract Syntax Tree for the Hybrid language

/// Represents a type in the Hybrid type system
#[derive(Debug, Clone, PartialEq)]
pub enum HybridType {
    Int,
    Float,
    String,
    Bool,
    Void,
    Null,
    Array(Box<HybridType>),           // array[int]
    Map(Box<HybridType>, Box<HybridType>), // map{string, int}
}

/// A typed parameter: (name, type)
#[derive(Debug, Clone, PartialEq)]
pub struct TypedParam {
    pub name: String,
    pub param_type: HybridType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Boolean(bool),
    StringLiteral(String),
    Identifier(String),
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Array(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    VariableDeclaration {
        is_const: bool,
        name: String,
        var_type: HybridType,
        value: Expr,
    },
    BlockDeclaration {
        name: String,
        parameters: Vec<TypedParam>,
        return_types: Vec<HybridType>,
        body: Vec<Stmt>,
        is_foreign: bool,               // true if preceded by #lang
        foreign_lang: Option<String>,   // e.g., "python", "rust"
        raw_body: Option<String>,       // raw code for foreign blocks
    },
    Return(Option<Expr>),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
