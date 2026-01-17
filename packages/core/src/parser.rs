use anyhow::{bail, Context};

use crate::ast::{Block, Expr, Lit, ObjKey, Param, Program, Span, Stmt, TypeName};
use crate::lexer::{lex, Kind, Token};

#[derive(Debug)]
pub struct Parser {
    filename: String,
    tokens: Vec<Token>,
    i: usize,
}

pub fn parse_program(src: &str, filename: &str) -> anyhow::Result<Program> {
    let tokens = lex(src)?;
    let mut p = Parser {
        filename: filename.to_string(),
        tokens,
        i: 0,
    };
    p.parse_program()
}

impl Parser {
    fn cur(&self) -> &Token {
        &self.tokens[self.i]
    }

    fn at(&self, kind: Kind) -> bool {
        self.cur().kind == kind
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.i].clone();
        self.i += 1;
        t
    }

    fn span(&self, t: &Token) -> Span {
        Span::new(&self.filename, t.line, t.col)
    }

    fn expect(&mut self, kind: Kind) -> anyhow::Result<Token> {
        if !self.at(kind.clone()) {
            bail!(
                "{}:{}:{}: Expected {:?}, got {:?} {:?}",
                self.filename,
                self.cur().line,
                self.cur().col,
                kind,
                self.cur().kind,
                self.cur().value
            );
        }
        Ok(self.advance())
    }

    fn match_kind(&mut self, kind: Kind) -> Option<Token> {
        if self.at(kind) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn skip_newlines(&mut self) {
        while self.at(Kind::Newline) {
            self.advance();
        }
    }

    fn parse_program(&mut self) -> anyhow::Result<Program> {
        self.skip_newlines();
        let start = self.cur().clone();
        let mut stmts = Vec::new();
        while !self.at(Kind::Eof) {
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }
        Ok(Program {
            span: self.span(&start),
            statements: stmts,
        })
    }

    fn parse_block(&mut self) -> anyhow::Result<Block> {
        let lb = self.expect(Kind::LBrace)?;
        self.skip_newlines();
        let mut stmts = Vec::new();
        while !self.at(Kind::RBrace) {
            if self.at(Kind::Eof) {
                bail!(
                    "{}:{}:{}: Unterminated block",
                    self.filename,
                    self.cur().line,
                    self.cur().col
                );
            }
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }
        self.expect(Kind::RBrace)?;
        Ok(Block {
            span: self.span(&lb),
            statements: stmts,
        })
    }

    fn peek_is_in(&self) -> bool {
        if self.cur().kind != Kind::Ident {
            return false;
        }
        if self.i + 1 >= self.tokens.len() {
            return false;
        }
        let nxt = &self.tokens[self.i + 1];
        nxt.kind == Kind::Ident && nxt.value == "in"
    }

    fn parse_stmt(&mut self) -> anyhow::Result<Stmt> {
        self.skip_newlines();
        let t = self.cur().clone();

        if self.at(Kind::Import) {
            self.advance();
            // import "path"  OR  import name from "path"
            if self.at(Kind::Str) {
                let p = self.advance().value;
                return Ok(Stmt::Import {
                    span: self.span(&t),
                    name: None,
                    path: p,
                });
            }
            let name = self.expect(Kind::Ident)?.value;
            self.expect(Kind::From)?;
            let path = self.expect(Kind::Str)?.value;
            return Ok(Stmt::Import {
                span: self.span(&t),
                name: Some(name),
                path,
            });
        }

        if self.at(Kind::Export) {
            self.advance();
            // export <decl> OR export <name>
            if self.at(Kind::Ayo)
                || self.at(Kind::Let)
                || self.at(Kind::Yoo)
                || self.at(Kind::Const)
                || self.at(Kind::Bruh)
                || self.at(Kind::Function)
            {
                // parse the declaration directly (without re-consuming export)
                let decl = self.parse_stmt()?;
                return Ok(Stmt::ExportDecl {
                    span: self.span(&t),
                    decl: Box::new(decl),
                });
            }
            let name = self.expect(Kind::Ident)?.value;
            return Ok(Stmt::Export {
                span: self.span(&t),
                names: vec![name],
            });
        }

        if self.at(Kind::Ayo) || self.at(Kind::Let) {
            self.advance();
            let name = self.expect(Kind::Ident)?;
            let ty = self.parse_type_annotation()?;
            self.expect(Kind::Eq)?;
            let value = self.parse_expr(0, &[])?;
            return Ok(Stmt::VarDecl {
                span: self.span(&t),
                name: name.value,
                ty,
                value,
            });
        }

        if self.at(Kind::Yoo) || self.at(Kind::Const) {
            self.advance();
            let name = self.expect(Kind::Ident)?;
            let ty = self.parse_type_annotation()?;
            self.expect(Kind::Eq)?;
            let value = self.parse_expr(0, &[])?;
            return Ok(Stmt::ConstDecl {
                span: self.span(&t),
                name: name.value,
                ty,
                value,
            });
        }

        if self.at(Kind::Bruh) || self.at(Kind::Function) {
            self.advance();
            let name = self.expect(Kind::Ident)?;
            self.expect(Kind::LParen)?;
            let mut params: Vec<Param> = Vec::new();
            if !self.at(Kind::RParen) {
                loop {
                    let p = self.expect(Kind::Ident)?;
                    let ty = self.parse_type_annotation()?;
                    params.push(Param {
                        span: self.span(&p),
                        name: p.value,
                        ty,
                    });
                    if self.match_kind(Kind::Comma).is_none() {
                        break;
                    }
                }
            }
            self.expect(Kind::RParen)?;
            let is_async = self.match_kind(Kind::HawkTuah).is_some();
            let ret_ty = self.parse_type_annotation()?;
            let body = self.parse_block()?;
            return Ok(Stmt::FuncDef {
                span: self.span(&t),
                name: name.value,
                params,
                is_async,
                ret_ty,
                body,
            });
        }

        if self.at(Kind::Return) {
            self.advance();
            // return <expr>? (expr optional)
            if self.at(Kind::Newline) || self.at(Kind::RBrace) || self.at(Kind::Eof) {
                return Ok(Stmt::Return {
                    span: self.span(&t),
                    value: None,
                });
            }
            let value = self.parse_expr(0, &[])?;
            return Ok(Stmt::Return {
                span: self.span(&t),
                value: Some(value),
            });
        }

        if self.at(Kind::Throw) {
            self.advance();
            let value = self.parse_expr(0, &[])?;
            return Ok(Stmt::Throw {
                span: self.span(&t),
                value,
            });
        }

        if self.at(Kind::Try) {
            self.advance();
            let try_block = self.parse_block()?;

            self.skip_newlines();
            let mut catch_name: Option<String> = None;
            let mut catch_block: Option<Block> = None;
            if self.match_kind(Kind::Catch).is_some() {
                self.skip_newlines();
                self.expect(Kind::LParen)?;
                let name = self.expect(Kind::Ident)?;
                self.expect(Kind::RParen)?;
                let blk = self.parse_block()?;
                catch_name = Some(name.value);
                catch_block = Some(blk);
            }

            self.skip_newlines();
            let mut finally_block: Option<Block> = None;
            if self.match_kind(Kind::Finally).is_some() {
                let blk = self.parse_block()?;
                finally_block = Some(blk);
            }

            return Ok(Stmt::Try {
                span: self.span(&t),
                try_block,
                catch_name,
                catch_block,
                finally_block,
            });
        }

        if self.at(Kind::Maybe) {
            self.advance();
            let cond = self.parse_expr(0, &[Kind::LBrace])?;
            let then_block = self.parse_block()?;
            let mut elifs = Vec::new();
            let mut else_block = None;
            loop {
                self.skip_newlines();
                if self.match_kind(Kind::Unless).is_none() {
                    break;
                }
                self.skip_newlines();
                if self.match_kind(Kind::Maybe).is_some() {
                    let econd = self.parse_expr(0, &[Kind::LBrace])?;
                    let eblk = self.parse_block()?;
                    elifs.push((econd, eblk));
                    continue;
                }
                else_block = Some(self.parse_block()?);
                break;
            }
            return Ok(Stmt::IfChain {
                span: self.span(&t),
                cond,
                then_block,
                elifs,
                else_block,
            });
        }

        if self.at(Kind::Crazy) {
            self.advance();
            if self.peek_is_in() {
                let var = self.expect(Kind::Ident)?.value;
                self.advance(); // 'in'
                let iterable = self.parse_expr(0, &[Kind::LBrace])?;
                let body = self.parse_block()?;
                return Ok(Stmt::ForIn {
                    span: self.span(&t),
                    var,
                    iterable,
                    body,
                });
            }
            let cond = self.parse_expr(0, &[Kind::LBrace])?;
            let body = self.parse_block()?;
            return Ok(Stmt::WhileLoop {
                span: self.span(&t),
                cond,
                body,
            });
        }

        if self.at(Kind::Rizz) {
            self.advance();
            self.expect(Kind::LParen)?;
            let value = self.parse_expr(0, &[Kind::RParen])?;
            self.expect(Kind::RParen)?;
            return Ok(Stmt::Rizz {
                span: self.span(&t),
                value,
            });
        }

        if self.at(Kind::Cringe) {
            self.advance();
            self.expect(Kind::LParen)?;
            let value = self.parse_expr(0, &[Kind::RParen])?;
            self.expect(Kind::RParen)?;
            return Ok(Stmt::Cringe {
                span: self.span(&t),
                value,
            });
        }

        // assignment: IDENT = expr
        if self.at(Kind::Ident)
            && self.i + 1 < self.tokens.len()
            && self.tokens[self.i + 1].kind == Kind::Eq
        {
            let name = self.advance();
            self.advance(); // '='
            let value = self.parse_expr(0, &[])?;
            return Ok(Stmt::Assign {
                span: self.span(&t),
                name: name.value,
                value,
            });
        }

        let expr = self.parse_expr(0, &[])?;
        Ok(Stmt::ExprStmt {
            span: self.span(&t),
            expr,
        })
    }

    fn parse_expr(&mut self, min_bp: u8, stop: &[Kind]) -> anyhow::Result<Expr> {
        for k in stop {
            if self.at(k.clone()) {
                bail!(
                    "{}:{}:{}: Unexpected token in expression",
                    self.filename,
                    self.cur().line,
                    self.cur().col
                );
            }
        }

        let mut left = self.parse_prefix(stop)?;

        loop {
            if stop.iter().any(|k| self.at(k.clone())) {
                break;
            }

            // postfix
            if self.at(Kind::Dot) {
                let dot = self.advance();
                let name = self.expect(Kind::Ident)?;
                left = Expr::Member {
                    span: self.span(&dot),
                    obj: Box::new(left),
                    name: name.value,
                };
                continue;
            }
            if self.at(Kind::LBracket) {
                let lb = self.advance();
                let idx = self.parse_expr(0, &[Kind::RBracket])?;
                self.expect(Kind::RBracket)?;
                left = Expr::Index {
                    span: self.span(&lb),
                    obj: Box::new(left),
                    index: Box::new(idx),
                };
                continue;
            }
            if self.at(Kind::LParen) {
                let lp = self.advance();
                let mut args = Vec::new();
                if !self.at(Kind::RParen) {
                    loop {
                        args.push(self.parse_expr(0, &[Kind::Comma, Kind::RParen])?);
                        if self.match_kind(Kind::Comma).is_none() {
                            break;
                        }
                    }
                }
                self.expect(Kind::RParen)?;
                left = Expr::Call {
                    span: self.span(&lp),
                    callee: Box::new(left),
                    args,
                };
                continue;
            }

            // infix
            let op_kind = self.cur().kind.clone();
            let (lbp, rbp, op_str) = match op_kind {
                Kind::OrOr => (10, 11, "||"),
                Kind::AndAnd => (20, 21, "&&"),
                Kind::EqEq => (30, 31, "=="),
                Kind::Neq => (30, 31, "!="),
                Kind::Gt => (40, 41, ">"),
                Kind::Lt => (40, 41, "<"),
                Kind::Gte => (40, 41, ">="),
                Kind::Lte => (40, 41, "<="),
                Kind::DotDot => (45, 46, ".."),
                Kind::Plus => (50, 51, "+"),
                Kind::Minus => (50, 51, "-"),
                Kind::Star => (60, 61, "*"),
                Kind::Slash => (60, 61, "/"),
                Kind::Percent => (60, 61, "%"),
                _ => break,
            };
            if lbp < min_bp {
                break;
            }
            let op_tok = self.advance();
            let right = self.parse_expr(rbp, stop)?;

            left = if op_str == ".." {
                Expr::Range {
                    span: self.span(&op_tok),
                    start: Box::new(left),
                    end: Box::new(right),
                }
            } else {
                Expr::Binary {
                    span: self.span(&op_tok),
                    op: op_str.to_string(),
                    left: Box::new(left),
                    right: Box::new(right),
                }
            };
        }

        Ok(left)
    }

    fn parse_prefix(&mut self, stop: &[Kind]) -> anyhow::Result<Expr> {
        let t = self.cur().clone();

        if self.at(Kind::Maybe) {
            // ternary: Maybe <cond> ? <a> : <b>
            self.advance();
            let cond = self.parse_expr(0, &[Kind::Qmark])?;
            self.expect(Kind::Qmark)?;
            let if_true = self.parse_expr(0, &[Kind::Colon])?;
            self.expect(Kind::Colon)?;
            let if_false = self.parse_expr(0, stop)?;
            return Ok(Expr::Ternary {
                span: self.span(&t),
                cond: Box::new(cond),
                if_true: Box::new(if_true),
                if_false: Box::new(if_false),
            });
        }

        if self.at(Kind::Vibe) {
            self.advance();
            let expr = self.parse_expr(80, stop)?;
            return Ok(Expr::Vibe {
                span: self.span(&t),
                expr: Box::new(expr),
            });
        }

        if self.at(Kind::Attempt) {
            self.advance();
            let try_block = self.parse_block()?;
            self.skip_newlines();
            self.expect(Kind::Eww)?;
            self.expect(Kind::LParen)?;
            let err = self.expect(Kind::Ident)?;
            self.expect(Kind::RParen)?;
            let catch_block = self.parse_block()?;
            return Ok(Expr::Attempt {
                span: self.span(&t),
                try_block,
                err_name: err.value,
                catch_block,
            });
        }

        if self.at(Kind::Bang) || self.at(Kind::Minus) {
            let op = self.advance();
            let expr = self.parse_expr(70, stop)?;
            return Ok(Expr::Unary {
                span: self.span(&op),
                op: op.value,
                expr: Box::new(expr),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> anyhow::Result<Expr> {
        let t = self.cur().clone();
        let span = self.span(&t);

        match t.kind {
            Kind::Int => {
                self.advance();
                let v: i64 = t.value.parse().context("invalid int")?;
                Ok(Expr::Literal {
                    span,
                    lit: Lit::Int(v),
                })
            }
            Kind::Float => {
                self.advance();
                let v: f64 = t.value.parse().context("invalid float")?;
                Ok(Expr::Literal {
                    span,
                    lit: Lit::Float(v),
                })
            }
            Kind::Str => {
                self.advance();
                Ok(Expr::Literal {
                    span,
                    lit: Lit::Str(t.value),
                })
            }
            Kind::Regex => {
                self.advance();
                Ok(Expr::Literal {
                    span,
                    lit: Lit::Regex(t.value),
                })
            }
            Kind::Char => {
                self.advance();
                let c = t.value.chars().next().unwrap_or('\0');
                Ok(Expr::Literal {
                    span,
                    lit: Lit::Char(c),
                })
            }
            Kind::Ident => {
                self.advance();
                match t.value.as_str() {
                    "null" => Ok(Expr::Literal {
                        span,
                        lit: Lit::Null,
                    }),
                    "true" => Ok(Expr::Literal {
                        span,
                        lit: Lit::Bool(true),
                    }),
                    "false" => Ok(Expr::Literal {
                        span,
                        lit: Lit::Bool(false),
                    }),
                    _ => Ok(Expr::Ident {
                        span,
                        name: t.value,
                    }),
                }
            }
            Kind::LParen => {
                self.advance();
                let e = self.parse_expr(0, &[Kind::RParen])?;
                self.expect(Kind::RParen)?;
                Ok(e)
            }
            Kind::LBracket => {
                self.advance();
                let mut items = Vec::new();
                self.skip_newlines();
                if !self.at(Kind::RBracket) {
                    loop {
                        items.push(self.parse_expr(0, &[Kind::Comma, Kind::RBracket])?);
                        self.skip_newlines();
                        if self.match_kind(Kind::Comma).is_none() {
                            break;
                        }
                        self.skip_newlines();
                    }
                }
                self.expect(Kind::RBracket)?;
                Ok(Expr::Array { span, items })
            }
            Kind::LBrace => {
                self.advance();
                let mut items = Vec::new();
                self.skip_newlines();
                if !self.at(Kind::RBrace) {
                    loop {
                        let key = if self.at(Kind::Ident) {
                            let k = self.advance();
                            ObjKey::Ident(k.value)
                        } else {
                            let kexpr = self.parse_expr(0, &[Kind::Colon])?;
                            ObjKey::Expr(Box::new(kexpr))
                        };
                        self.expect(Kind::Colon)?;
                        let val = self.parse_expr(0, &[Kind::Comma, Kind::RBrace])?;
                        items.push((key, val));
                        self.skip_newlines();
                        if self.match_kind(Kind::Comma).is_none() {
                            break;
                        }
                        self.skip_newlines();
                    }
                }
                self.expect(Kind::RBrace)?;
                Ok(Expr::Object { span, items })
            }
            _ => bail!(
                "{}:{}:{}: Expected expression, got {:?} {:?}",
                self.filename,
                t.line,
                t.col,
                t.kind,
                t.value
            ),
        }
    }

    fn parse_type_annotation(&mut self) -> anyhow::Result<Option<TypeName>> {
        if self.match_kind(Kind::Colon).is_none() {
            return Ok(None);
        }
        // allow optional whitespace/newlines after :
        self.skip_newlines();
        let t = self.expect(Kind::Ident)?;
        Ok(Some(TypeName {
            span: self.span(&t),
            name: t.value,
        }))
    }
}
