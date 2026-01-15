use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;

use anyhow::{anyhow, bail};
use regex::Regex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;

use crate::ast::{Block, Expr, Lit, ObjKey, Program, Stmt};

#[derive(Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
    Regex(String),

    Function(Rc<dyn Callable>),
    Task(Rc<RefCell<Option<JoinHandle<anyhow::Result<Value>>>>>),
    Socket(Rc<tokio::sync::Mutex<TcpStream>>),
    Server(Rc<RefCell<Option<JoinHandle<anyhow::Result<()>>>>>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Bool(b) => write!(f, "Bool({b})"),
            Value::Int(i) => write!(f, "Int({i})"),
            Value::Float(x) => write!(f, "Float({x})"),
            Value::Str(s) => write!(f, "Str({s:?})"),
            Value::Array(a) => write!(f, "Array(len={})", a.len()),
            Value::Object(o) => write!(f, "Object(len={})", o.len()),
            Value::Regex(p) => write!(f, "Regex({p:?})"),
            Value::Function(_) => write!(f, "Function(..)"),
            Value::Task(_) => write!(f, "Task(..)"),
            Value::Socket(_) => write!(f, "Socket(..)"),
            Value::Server(_) => write!(f, "Server(..)"),
        }
    }
}

impl Value {
    fn truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
            _ => true,
        }
    }

    fn as_string(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => {
                let mut s = f.to_string();
                if s.ends_with(".0") {
                    s.truncate(s.len() - 2);
                }
                s
            }
            Value::Str(s) => s.clone(),
            Value::Array(_) | Value::Object(_) => match to_json(self) {
                Ok(v) => v.to_string(),
                Err(_) => format!("{self:?}"),
            },
            Value::Regex(p) => format!("r\"{p}\""),
            Value::Function(_) => "<function>".to_string(),
            Value::Task(_) => "<task>".to_string(),
            Value::Socket(_) => "<socket>".to_string(),
            Value::Server(_) => "<server>".to_string(),
        }
    }
}

#[derive(Default, Clone)]
pub struct Env {
    parent: Option<Box<Env>>,
    values: BTreeMap<String, Value>,
    consts: BTreeMap<String, bool>,
}

impl Env {
    fn child(&self) -> Env {
        Env {
            parent: Some(Box::new(self.clone())),
            values: BTreeMap::new(),
            consts: BTreeMap::new(),
        }
    }

    fn define(&mut self, name: &str, value: Value, is_const: bool) {
        self.values.insert(name.to_string(), value);
        if is_const {
            self.consts.insert(name.to_string(), true);
        }
    }

    fn get(&self, name: &str) -> anyhow::Result<Value> {
        if let Some(v) = self.values.get(name) {
            return Ok(v.clone());
        }
        if let Some(p) = &self.parent {
            return p.get(name);
        }
        bail!("Undefined variable: {name}");
    }

    fn set(&mut self, name: &str, value: Value) -> anyhow::Result<()> {
        if self.values.contains_key(name) {
            if self.consts.get(name).copied().unwrap_or(false) {
                bail!("Cannot assign to constant: {name}");
            }
            self.values.insert(name.to_string(), value);
            return Ok(());
        }
        if let Some(p) = &mut self.parent {
            return p.set(name, value);
        }
        bail!("Undefined variable: {name}");
    }
}

#[async_trait::async_trait(?Send)]
pub trait Callable {
    async fn call(&self, rt: &mut Runtime, args: Vec<Value>) -> anyhow::Result<Value>;
}

#[derive(Copy, Clone)]
enum BuiltinKind {
    Chill,
    Spit,
    Yeet,
    Flex,
    Ghost,
    Decode,
    Encode,
    Hunt,
    Swap,
    Matches,
    Split,
    Snag,
    Stash,
    KeepAdding,
    Trash,
    FileExists,
    Listen,
    Holla,
    Peek,
    Whisper,
    Dip,
}

struct Builtin {
    kind: BuiltinKind,
}

#[async_trait::async_trait(?Send)]
impl Callable for Builtin {
    async fn call(&self, rt: &mut Runtime, args: Vec<Value>) -> anyhow::Result<Value> {
        match self.kind {
            BuiltinKind::Chill => b_chill(args).await,
            BuiltinKind::Spit => b_spit(rt, args).await,
            BuiltinKind::Yeet => b_yeet(rt, args).await,
            BuiltinKind::Flex => b_flex(rt, args).await,
            BuiltinKind::Ghost => b_ghost(rt, args).await,
            BuiltinKind::Decode => b_decode(args).await,
            BuiltinKind::Encode => b_encode(args).await,
            BuiltinKind::Hunt => b_hunt(args).await,
            BuiltinKind::Swap => b_swap(args).await,
            BuiltinKind::Matches => b_matches(args).await,
            BuiltinKind::Split => b_split(args).await,
            BuiltinKind::Snag => b_snag(args).await,
            BuiltinKind::Stash => b_stash(args).await,
            BuiltinKind::KeepAdding => b_keepadding(args).await,
            BuiltinKind::Trash => b_trash(args).await,
            BuiltinKind::FileExists => b_fileexists(args).await,
            BuiltinKind::Listen => b_listen(args).await,
            BuiltinKind::Holla => b_holla(args).await,
            BuiltinKind::Peek => b_peek(args).await,
            BuiltinKind::Whisper => b_whisper(args).await,
            BuiltinKind::Dip => b_dip(args).await,
        }
    }
}

#[derive(Clone)]
struct UserFunction {
    name: String,
    params: Vec<String>,
    body: Block,
    closure: Env,
}

#[async_trait::async_trait(?Send)]
impl Callable for UserFunction {
    async fn call(&self, rt: &mut Runtime, args: Vec<Value>) -> anyhow::Result<Value> {
        if args.len() != self.params.len() {
            bail!(
                "{}() expected {} args, got {}",
                self.name,
                self.params.len(),
                args.len()
            );
        }
        let mut env = self.closure.child();
        for (k, v) in self.params.iter().zip(args.into_iter()) {
            env.define(k, v, false);
        }
        let mut frame = Frame::default();
        rt.exec_block(&self.body, &mut env, &mut frame).await?;
        Ok(frame.return_value.unwrap_or(Value::Null))
    }
}

#[derive(Default)]
struct Frame {
    return_value: Option<Value>,
}

pub struct Runtime {
    filename: String,
    globals: Env,
    client: reqwest::Client,
    tasks: Vec<Rc<RefCell<Option<JoinHandle<anyhow::Result<Value>>>>>>,
}

impl Runtime {
    pub fn new(filename: &str) -> Self {
        let mut rt = Self {
            filename: filename.to_string(),
            globals: Env::default(),
            client: reqwest::Client::new(),
            tasks: Vec::new(),
        };
        rt.install_builtins();
        rt
    }

    fn install_builtins(&mut self) {
        self.globals.define("Chill", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Chill })), true);
        self.globals.define("Spit", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Spit })), true);
        self.globals.define("Yeet", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Yeet })), true);
        self.globals.define("Flex", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Flex })), true);
        self.globals.define("Ghost", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Ghost })), true);
        self.globals.define("Decode", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Decode })), true);
        self.globals.define("Encode", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Encode })), true);
        self.globals.define("Hunt", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Hunt })), true);
        self.globals.define("Swap", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Swap })), true);
        self.globals.define("Matches", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Matches })), true);
        self.globals.define("Split", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Split })), true);
        self.globals.define("Snag", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Snag })), true);
        self.globals.define("Stash", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Stash })), true);
        self.globals.define(
            "KeepAdding",
            Value::Function(Rc::new(Builtin { kind: BuiltinKind::KeepAdding })),
            true,
        );
        self.globals.define("Trash", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Trash })), true);
        self.globals.define(
            "FileExists",
            Value::Function(Rc::new(Builtin { kind: BuiltinKind::FileExists })),
            true,
        );
        self.globals.define("Listen", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Listen })), true);
        self.globals.define("Holla", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Holla })), true);
        self.globals.define("Peek", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Peek })), true);
        self.globals.define(
            "Whisper",
            Value::Function(Rc::new(Builtin { kind: BuiltinKind::Whisper })),
            true,
        );
        self.globals.define("Dip", Value::Function(Rc::new(Builtin { kind: BuiltinKind::Dip })), true);
    }

    pub async fn exec_program(&mut self, program: &Program) -> anyhow::Result<()> {
        let mut frame = Frame::default();
        let mut globals = std::mem::take(&mut self.globals);
        for s in &program.statements {
            self.exec_stmt(s, &mut globals, &mut frame).await?;
        }
        // Await any "fire-and-forget" tasks spawned by `Vibe` that weren't `Chill`'d.
        for t in self.tasks.iter() {
            if let Some(handle) = t.borrow_mut().take() {
                let _ = handle.await??;
            }
        }
        self.globals = globals;
        Ok(())
    }

    #[async_recursion::async_recursion(?Send)]
    async fn exec_block(&mut self, block: &Block, env: &mut Env, frame: &mut Frame) -> anyhow::Result<()> {
        for s in &block.statements {
            self.exec_stmt(s, env, frame).await?;
        }
        Ok(())
    }

    #[async_recursion::async_recursion(?Send)]
    async fn exec_stmt(&mut self, s: &Stmt, env: &mut Env, frame: &mut Frame) -> anyhow::Result<()> {
        match s {
            Stmt::VarDecl { name, value, .. } => {
                let v = self.eval_expr(value, env, frame).await?;
                env.define(name, v, false);
            }
            Stmt::ConstDecl { name, value, .. } => {
                let v = self.eval_expr(value, env, frame).await?;
                env.define(name, v, true);
            }
            Stmt::Assign { name, value, .. } => {
                let v = self.eval_expr(value, env, frame).await?;
                env.set(name, v)?;
            }
            Stmt::FuncDef { name, params, body, .. } => {
                let f = UserFunction {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: env.clone(),
                };
                env.define(name, Value::Function(Rc::new(f)), true);
            }
            Stmt::Rizz { value, .. } => {
                let v = self.eval_expr(value, env, frame).await?;
                frame.return_value = Some(v.clone());
                println!("{}", v.as_string());
            }
            Stmt::Cringe { value, .. } => {
                let v = self.eval_expr(value, env, frame).await?;
                bail!("{}", v.as_string());
            }
            Stmt::ExprStmt { expr, .. } => {
                let _ = self.eval_expr(expr, env, frame).await?;
            }
            Stmt::IfChain { cond, then_block, elifs, else_block, .. } => {
                if self.eval_expr(cond, env, frame).await?.truthy() {
                    let mut child = env.child();
                    self.exec_block(then_block, &mut child, frame).await?;
                } else {
                    let mut done = false;
                    for (c, b) in elifs {
                        if self.eval_expr(c, env, frame).await?.truthy() {
                            let mut child = env.child();
                            self.exec_block(b, &mut child, frame).await?;
                            done = true;
                            break;
                        }
                    }
                    if !done {
                        if let Some(b) = else_block {
                            let mut child = env.child();
                            self.exec_block(b, &mut child, frame).await?;
                        }
                    }
                }
            }
            Stmt::ForIn { var, iterable, body, .. } => {
                let it = self.eval_expr(iterable, env, frame).await?;
                let list = match it {
                    Value::Array(a) => a,
                    _ => bail!("Crazy ... in ... expects an array"),
                };
                for v in list {
                    let mut child = env.child();
                    child.define(var, v, false);
                    self.exec_block(body, &mut child, frame).await?;
                }
            }
            Stmt::WhileLoop { cond, body, .. } => {
                while self.eval_expr(cond, env, frame).await?.truthy() {
                    let mut child = env.child();
                    self.exec_block(body, &mut child, frame).await?;
                }
            }
        }
        Ok(())
    }

    #[async_recursion::async_recursion(?Send)]
    async fn eval_expr(&mut self, e: &Expr, env: &mut Env, frame: &mut Frame) -> anyhow::Result<Value> {
        match e {
            Expr::Ident { name, .. } => env.get(name),
            Expr::Literal { lit, .. } => Ok(match lit {
                Lit::Null => Value::Null,
                Lit::Bool(b) => Value::Bool(*b),
                Lit::Int(i) => Value::Int(*i),
                Lit::Float(f) => Value::Float(*f),
                Lit::Char(c) => Value::Str(c.to_string()),
                Lit::Str(s) => Value::Str(s.clone()),
                Lit::Regex(p) => Value::Regex(p.clone()),
            }),
            Expr::Array { items, .. } => {
                let mut out = Vec::new();
                for x in items {
                    out.push(self.eval_expr(x, env, frame).await?);
                }
                Ok(Value::Array(out))
            }
            Expr::Object { items, .. } => {
                let mut out = BTreeMap::new();
                for (k, v) in items {
                    let key = match k {
                        ObjKey::Ident(s) => s.clone(),
                        ObjKey::Expr(ex) => self.eval_expr(ex, env, frame).await?.as_string(),
                    };
                    out.insert(key, self.eval_expr(v, env, frame).await?);
                }
                Ok(Value::Object(out))
            }
            Expr::Unary { op, expr, .. } => {
                let v = self.eval_expr(expr, env, frame).await?;
                match op.as_str() {
                    "!" => Ok(Value::Bool(!v.truthy())),
                    "-" => match v {
                        Value::Int(i) => Ok(Value::Int(-i)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        _ => bail!("Unary - expects a number"),
                    },
                    _ => bail!("Unknown unary op: {op}"),
                }
            }
            Expr::Binary { op, left, right, .. } => {
                let l = self.eval_expr(left, env, frame).await?;
                let r = self.eval_expr(right, env, frame).await?;
                binop(op, l, r)
            }
            Expr::Range { start, end, .. } => {
                let a = self.eval_expr(start, env, frame).await?;
                let b = self.eval_expr(end, env, frame).await?;
                let (a, b) = (to_i64(&a)?, to_i64(&b)?);
                let mut out = Vec::new();
                if a <= b {
                    for i in a..=b {
                        out.push(Value::Int(i));
                    }
                } else {
                    let mut i = a;
                    while i >= b {
                        out.push(Value::Int(i));
                        if i == b {
                            break;
                        }
                        i -= 1;
                    }
                }
                Ok(Value::Array(out))
            }
            Expr::Ternary { cond, if_true, if_false, .. } => {
                if self.eval_expr(cond, env, frame).await?.truthy() {
                    self.eval_expr(if_true, env, frame).await
                } else {
                    self.eval_expr(if_false, env, frame).await
                }
            }
            Expr::Member { obj, name, .. } => {
                let o = self.eval_expr(obj, env, frame).await?;
                match o {
                    Value::Object(map) => Ok(map.get(name).cloned().unwrap_or(Value::Null)),
                    _ => bail!("Member access on non-object"),
                }
            }
            Expr::Index { obj, index, .. } => {
                let o = self.eval_expr(obj, env, frame).await?;
                let idx = self.eval_expr(index, env, frame).await?;
                match (o, idx) {
                    (Value::Array(a), Value::Int(i)) => {
                        let ui: usize = i.try_into().map_err(|_| anyhow!("index must be >= 0"))?;
                        Ok(a.get(ui).cloned().unwrap_or(Value::Null))
                    }
                    _ => bail!("Indexing expects array[int]"),
                }
            }
            Expr::Call { callee, args, .. } => {
                let c = self.eval_expr(callee, env, frame).await?;
                let mut ev_args = Vec::new();
                for a in args {
                    ev_args.push(self.eval_expr(a, env, frame).await?);
                }
                match c {
                    Value::Function(f) => f.call(self, ev_args).await,
                    _ => bail!("Cannot call non-function"),
                }
            }
            Expr::Vibe { expr, .. } => {
                let mut snap = env.clone();
                let e2 = (*expr.clone()).clone();
                let handle = tokio::task::spawn_local(async move {
                    let mut rt = Runtime::new("<task>");
                    let mut frame = Frame::default();
                    rt.eval_expr(&e2, &mut snap, &mut frame).await
                });
                let cell = Rc::new(RefCell::new(Some(handle)));
                self.tasks.push(cell.clone());
                Ok(Value::Task(cell))
            }
            Expr::Attempt { try_block, err_name, catch_block, .. } => {
                let mut child = env.child();
                let mut local_frame = Frame::default();
                let res = self.exec_block(try_block, &mut child, &mut local_frame).await;
                match res {
                    Ok(()) => Ok(local_frame.return_value.unwrap_or(Value::Null)),
                    Err(e) => {
                        let mut child2 = env.child();
                        child2.define(err_name, Value::Str(e.to_string()), false);
                        let mut f2 = Frame::default();
                        self.exec_block(catch_block, &mut child2, &mut f2).await?;
                        Ok(f2.return_value.unwrap_or(Value::Null))
                    }
                }
            }
        }
    }
}

fn to_i64(v: &Value) -> anyhow::Result<i64> {
    match v {
        Value::Int(i) => Ok(*i),
        Value::Float(f) => Ok(*f as i64),
        Value::Str(s) => Ok(s.parse::<i64>()?),
        _ => bail!("Expected int"),
    }
}

fn binop(op: &str, l: Value, r: Value) -> anyhow::Result<Value> {
    match op {
        "&&" => Ok(Value::Bool(l.truthy() && r.truthy())),
        "||" => Ok(Value::Bool(l.truthy() || r.truthy())),
        "==" => Ok(Value::Bool(eq_val(&l, &r))),
        "!=" => Ok(Value::Bool(!eq_val(&l, &r))),
        ">" => Ok(Value::Bool(cmp_num(&l, &r)? > 0)),
        "<" => Ok(Value::Bool(cmp_num(&l, &r)? < 0)),
        ">=" => Ok(Value::Bool(cmp_num(&l, &r)? >= 0)),
        "<=" => Ok(Value::Bool(cmp_num(&l, &r)? <= 0)),
        "+" => match (l, r) {
            (Value::Str(a), b) => Ok(Value::Str(a + &b.as_string())),
            (a, Value::Str(b)) => Ok(Value::Str(a.as_string() + &b)),
            (Value::Array(mut a), Value::Array(b)) => {
                a.extend(b);
                Ok(Value::Array(a))
            }
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
            _ => bail!("Unsupported + operands"),
        },
        "-" => match (l, r) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
            _ => bail!("Unsupported - operands"),
        },
        "*" => match (l, r) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
            _ => bail!("Unsupported * operands"),
        },
        "/" => match (l, r) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Float(a as f64 / b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 / b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / b as f64)),
            _ => bail!("Unsupported / operands"),
        },
        "%" => match (l, r) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
            _ => bail!("Unsupported % operands"),
        },
        _ => bail!("Unknown operator: {op}"),
    }
}

fn eq_val(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Int(x), Value::Int(y)) => x == y,
        (Value::Float(x), Value::Float(y)) => x == y,
        (Value::Str(x), Value::Str(y)) => x == y,
        _ => false,
    }
}

fn cmp_num(a: &Value, b: &Value) -> anyhow::Result<i32> {
    let af = match a {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => bail!("Comparison expects numbers"),
    };
    let bf = match b {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => bail!("Comparison expects numbers"),
    };
    Ok(if af < bf { -1 } else if af > bf { 1 } else { 0 })
}

fn to_json(v: &Value) -> anyhow::Result<serde_json::Value> {
    Ok(match v {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Int(i) => serde_json::Value::Number((*i).into()),
        Value::Float(f) => serde_json::Value::Number(
            serde_json::Number::from_f64(*f).ok_or_else(|| anyhow!("bad float"))?,
        ),
        Value::Str(s) => serde_json::Value::String(s.clone()),
        Value::Array(a) => serde_json::Value::Array(
            a.iter()
                .map(|x| to_json(x))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Value::Object(o) => {
            let mut map = serde_json::Map::new();
            for (k, v) in o.iter() {
                map.insert(k.clone(), to_json(v)?);
            }
            serde_json::Value::Object(map)
        }
        Value::Regex(p) => serde_json::Value::String(p.clone()),
        _ => serde_json::Value::String(v.as_string()),
    })
}

fn from_json(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::Str(n.to_string())
            }
        }
        serde_json::Value::String(s) => Value::Str(s),
        serde_json::Value::Array(a) => Value::Array(a.into_iter().map(from_json).collect()),
        serde_json::Value::Object(o) => {
            let mut map = BTreeMap::new();
            for (k, v) in o {
                map.insert(k, from_json(v));
            }
            Value::Object(map)
        }
    }
}

fn regex_pat(v: &Value) -> String {
    match v {
        Value::Regex(p) => p.clone(),
        Value::Str(s) => s.clone(),
        other => other.as_string(),
    }
}

// --- builtins ---

async fn b_chill(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 1 {
        bail!("Chill(task) expects 1 arg");
    }
    match &args[0] {
        Value::Task(h) => {
            let handle = h
                .borrow_mut()
                .take()
                .ok_or_else(|| anyhow!("Task already awaited"))?;
            handle.await?
        }
        Value::Server(h) => {
            let handle = h
                .borrow_mut()
                .take()
                .ok_or_else(|| anyhow!("Server already awaited"))?;
            handle.await??;
            Ok(Value::Null)
        }
        other => Ok(other.clone()),
    }
}

async fn http_req(rt: &Runtime, method: &str, url: &str, data: Option<Value>, headers: Option<Value>) -> anyhow::Result<Value> {
    let mut req = rt.client.request(method.parse()?, url);
    if let Some(Value::Object(h)) = headers {
        for (k, v) in h {
            req = req.header(k, v.as_string());
        }
    }
    if let Some(d) = data {
        match d {
            Value::Object(_) | Value::Array(_) => {
                let js = to_json(&d)?;
                req = req.json(&js);
            }
            other => req = req.body(other.as_string()),
        }
    }
    let resp = req.send().await?;
    let status = resp.status().as_u16() as i64;
    let body = resp.text().await?;
    let mut out = BTreeMap::new();
    out.insert("status".to_string(), Value::Int(status));
    out.insert("body".to_string(), Value::Str(body));
    Ok(Value::Object(out))
}

async fn b_spit(rt: &Runtime, args: Vec<Value>) -> anyhow::Result<Value> {
    if !(1..=2).contains(&args.len()) {
        bail!("Spit(url, headers?)");
    }
    let url = args[0].as_string();
    let headers = args.get(1).cloned();
    http_req(rt, "GET", &url, None, headers).await
}

async fn b_yeet(rt: &Runtime, args: Vec<Value>) -> anyhow::Result<Value> {
    if !(2..=3).contains(&args.len()) {
        bail!("Yeet(url, data, headers?)");
    }
    let url = args[0].as_string();
    let data = args[1].clone();
    let headers = args.get(2).cloned();
    http_req(rt, "POST", &url, Some(data), headers).await
}

async fn b_flex(rt: &Runtime, args: Vec<Value>) -> anyhow::Result<Value> {
    if !(2..=3).contains(&args.len()) {
        bail!("Flex(url, data, headers?)");
    }
    let url = args[0].as_string();
    let data = args[1].clone();
    let headers = args.get(2).cloned();
    http_req(rt, "PUT", &url, Some(data), headers).await
}

async fn b_ghost(rt: &Runtime, args: Vec<Value>) -> anyhow::Result<Value> {
    if !(1..=2).contains(&args.len()) {
        bail!("Ghost(url, headers?)");
    }
    let url = args[0].as_string();
    let headers = args.get(1).cloned();
    http_req(rt, "DELETE", &url, None, headers).await
}

async fn b_decode(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 1 {
        bail!("Decode(jsonString)");
    }
    let s = args[0].as_string();
    let v: serde_json::Value = serde_json::from_str(&s)?;
    Ok(from_json(v))
}

async fn b_encode(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 1 {
        bail!("Encode(obj)");
    }
    let v = to_json(&args[0])?;
    Ok(Value::Str(serde_json::to_string(&v)?))
}

async fn b_hunt(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 2 {
        bail!("Hunt(text, pattern)");
    }
    let text = args[0].as_string();
    let pat = regex_pat(&args[1]);
    let re = Regex::new(&pat)?;
    let out = re
        .find_iter(&text)
        .map(|m| Value::Str(m.as_str().to_string()))
        .collect();
    Ok(Value::Array(out))
}

async fn b_swap(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 3 {
        bail!("Swap(text, pattern, replacement)");
    }
    let text = args[0].as_string();
    let pat = regex_pat(&args[1]);
    let repl = args[2].as_string();
    let re = Regex::new(&pat)?;
    Ok(Value::Str(re.replace_all(&text, repl).to_string()))
}

async fn b_matches(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 2 {
        bail!("Matches(text, pattern)");
    }
    let text = args[0].as_string();
    let pat = regex_pat(&args[1]);
    let re = Regex::new(&pat)?;
    Ok(Value::Bool(re.is_match(&text)))
}

async fn b_split(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 2 {
        bail!("Split(text, pattern)");
    }
    let text = args[0].as_string();
    let pat = regex_pat(&args[1]);
    let re = Regex::new(&pat)?;
    Ok(Value::Array(
        re.split(&text).map(|s| Value::Str(s.to_string())).collect(),
    ))
}

async fn b_snag(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 1 {
        bail!("Snag(path)");
    }
    Ok(Value::Str(tokio::fs::read_to_string(args[0].as_string()).await?))
}

async fn b_stash(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 2 {
        bail!("Stash(path, text)");
    }
    tokio::fs::write(args[0].as_string(), args[1].as_string()).await?;
    Ok(Value::Null)
}

async fn b_keepadding(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 2 {
        bail!("KeepAdding(path, text)");
    }
    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(args[0].as_string())
        .await?;
    file.write_all(args[1].as_string().as_bytes()).await?;
    Ok(Value::Null)
}

async fn b_trash(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 1 {
        bail!("Trash(path)");
    }
    tokio::fs::remove_file(args[0].as_string()).await?;
    Ok(Value::Null)
}

async fn b_fileexists(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 1 {
        bail!("FileExists(path)");
    }
    Ok(Value::Bool(tokio::fs::try_exists(args[0].as_string()).await?))
}

async fn b_listen(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 2 {
        bail!("Listen(port, handler)");
    }
    let port = to_i64(&args[0])? as u16;
    let handler = match &args[1] {
        Value::Function(f) => f.clone(),
        _ => bail!("Listen expects function handler"),
    };
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    let handle = tokio::task::spawn_local(async move {
        let mut rt = Runtime::new("<server>");
        loop {
            let (stream, _) = listener.accept().await?;
            let sock = Value::Socket(Rc::new(tokio::sync::Mutex::new(stream)));
            let _ = handler.call(&mut rt, vec![sock]).await?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });
    Ok(Value::Server(Rc::new(RefCell::new(Some(handle)))))
}

async fn b_holla(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 2 {
        bail!("Holla(host, port)");
    }
    let host = args[0].as_string();
    let port = to_i64(&args[1])? as u16;
    let stream = TcpStream::connect((host.as_str(), port)).await?;
    Ok(Value::Socket(Rc::new(tokio::sync::Mutex::new(stream))))
}

async fn b_peek(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 1 {
        bail!("Peek(socket)");
    }
    let sock = match &args[0] {
        Value::Socket(s) => s.clone(),
        _ => bail!("Peek expects socket"),
    };
    let mut buf = vec![0u8; 4096];
    let mut s = sock.lock().await;
    let n = s.read(&mut buf).await?;
    buf.truncate(n);
    Ok(Value::Str(String::from_utf8_lossy(&buf).to_string()))
}

async fn b_whisper(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 2 {
        bail!("Whisper(socket, text)");
    }
    let sock = match &args[0] {
        Value::Socket(s) => s.clone(),
        _ => bail!("Whisper expects socket"),
    };
    let msg = args[1].as_string();
    let mut s = sock.lock().await;
    s.write_all(msg.as_bytes()).await?;
    Ok(Value::Null)
}

async fn b_dip(args: Vec<Value>) -> anyhow::Result<Value> {
    if args.len() != 1 {
        bail!("Dip(socket)");
    }
    let sock = match &args[0] {
        Value::Socket(s) => s.clone(),
        _ => bail!("Dip expects socket"),
    };
    let mut s = sock.lock().await;
    s.shutdown().await?;
    Ok(Value::Null)
}

