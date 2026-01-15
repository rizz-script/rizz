use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Span {
    pub filename: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub span: Span,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub span: Span,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl { span: Span, name: String, value: Expr },
    ConstDecl { span: Span, name: String, value: Expr },
    Assign { span: Span, name: String, value: Expr },
    FuncDef {
        span: Span,
        name: String,
        params: Vec<String>,
        is_async: bool,
        body: Block,
    },
    Rizz { span: Span, value: Expr },
    Cringe { span: Span, value: Expr },
    IfChain {
        span: Span,
        cond: Expr,
        then_block: Block,
        elifs: Vec<(Expr, Block)>,
        else_block: Option<Block>,
    },
    ForIn {
        span: Span,
        var: String,
        iterable: Expr,
        body: Block,
    },
    WhileLoop { span: Span, cond: Expr, body: Block },
    ExprStmt { span: Span, expr: Expr },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident { span: Span, name: String },
    Literal { span: Span, lit: Lit },
    Array { span: Span, items: Vec<Expr> },
    Object {
        span: Span,
        items: Vec<(ObjKey, Expr)>,
    },
    Unary {
        span: Span,
        op: String,
        expr: Box<Expr>,
    },
    Binary {
        span: Span,
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Ternary {
        span: Span,
        cond: Box<Expr>,
        if_true: Box<Expr>,
        if_false: Box<Expr>,
    },
    Call {
        span: Span,
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Member {
        span: Span,
        obj: Box<Expr>,
        name: String,
    },
    Index {
        span: Span,
        obj: Box<Expr>,
        index: Box<Expr>,
    },
    Vibe { span: Span, expr: Box<Expr> },
    Attempt {
        span: Span,
        try_block: Block,
        err_name: String,
        catch_block: Block,
    },
    Range {
        span: Span,
        start: Box<Expr>,
        end: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum ObjKey {
    Ident(String),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Lit {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    Str(String),
    Regex(String),
}

impl Span {
    pub fn new(filename: &str, line: usize, col: usize) -> Self {
        Self {
            filename: filename.to_string(),
            line,
            col,
        }
    }
}

pub type Object = BTreeMap<String, serde_json::Value>;

