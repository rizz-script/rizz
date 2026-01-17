// Simplified synchronous runtime for WASM
// This is a basic interpreter that works without tokio/async

use rizz_core::ast::{Block, Expr, Lit, Program, Stmt};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl Value {
    pub fn as_string(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Str(s) => s.clone(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.as_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.as_string()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }

    pub fn truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
        }
    }
}

pub struct SimpleRuntime {
    variables: BTreeMap<String, Value>,
    output: Vec<String>,
}

impl SimpleRuntime {
    pub fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
            output: Vec::new(),
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.execute_stmt(stmt)?;
        }
        Ok(())
    }

    pub fn get_output(&self) -> Vec<String> {
        self.output.clone()
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::VarDecl { name, value, .. } | Stmt::ConstDecl { name, value, .. } => {
                let val = self.eval_expr(value)?;
                self.variables.insert(name.clone(), val);
                Ok(())
            }
            Stmt::Assign { name, value, .. } => {
                let val = self.eval_expr(value)?;
                self.variables.insert(name.clone(), val);
                Ok(())
            }
            Stmt::Rizz { value, .. } => {
                let val = self.eval_expr(value)?;
                self.output.push(val.as_string());
                web_sys::console::log_1(&val.as_string().into());
                Ok(())
            }
            Stmt::IfChain {
                cond,
                then_block,
                else_block,
                ..
            } => {
                let cond_val = self.eval_expr(cond)?;
                if cond_val.truthy() {
                    self.execute_block(then_block)?;
                } else if let Some(else_blk) = else_block {
                    self.execute_block(else_blk)?;
                }
                Ok(())
            }
            Stmt::ForIn {
                var,
                iterable,
                body,
                ..
            } => {
                let iter_val = self.eval_expr(iterable)?;
                match iter_val {
                    Value::Array(arr) => {
                        for item in arr {
                            self.variables.insert(var.clone(), item);
                            self.execute_block(body)?;
                        }
                        Ok(())
                    }
                    _ => Err("Can only iterate over arrays".to_string()),
                }
            }
            Stmt::ExprStmt { expr, .. } => {
                self.eval_expr(expr)?;
                Ok(())
            }
            Stmt::FuncDef { .. } => {
                // Skip function definitions for now
                Ok(())
            }
            _ => Ok(()), // Skip other statements for now
        }
    }

    fn execute_block(&mut self, block: &Block) -> Result<(), String> {
        for stmt in &block.statements {
            self.execute_stmt(stmt)?;
        }
        Ok(())
    }

    fn eval_expr(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal { lit, .. } => match lit {
                Lit::Null => Ok(Value::Null),
                Lit::Bool(b) => Ok(Value::Bool(*b)),
                Lit::Int(i) => Ok(Value::Int(*i)),
                Lit::Float(f) => Ok(Value::Float(*f)),
                Lit::Str(s) => Ok(Value::Str(s.clone())),
                Lit::Char(c) => Ok(Value::Str(c.to_string())),
                Lit::Regex(r) => Ok(Value::Str(format!("r\"{}\"", r))),
            },
            Expr::Ident { name, .. } => self
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name)),
            Expr::Binary {
                left, op, right, ..
            } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binary(l, op, r)
            }
            Expr::Array { items, .. } => {
                let mut arr = Vec::new();
                for elem in items {
                    arr.push(self.eval_expr(elem)?);
                }
                Ok(Value::Array(arr))
            }
            Expr::Object { items, .. } => {
                let mut obj = BTreeMap::new();
                for (key, value) in items {
                    let key_str = match key {
                        rizz_core::ast::ObjKey::Ident(s) => s.clone(),
                        rizz_core::ast::ObjKey::Expr(e) => {
                            // Evaluate expression key
                            self.eval_expr(e)?.as_string()
                        }
                    };
                    obj.insert(key_str, self.eval_expr(value)?);
                }
                Ok(Value::Object(obj))
            }
            Expr::Index { obj, index, .. } => {
                let obj_val = self.eval_expr(obj)?;
                let idx = self.eval_expr(index)?;
                match (obj_val, idx) {
                    (Value::Array(arr), Value::Int(i)) => {
                        if i < 0 || i as usize >= arr.len() {
                            Err("Array index out of bounds".to_string())
                        } else {
                            Ok(arr[i as usize].clone())
                        }
                    }
                    (Value::Object(obj), Value::Str(key)) => {
                        Ok(obj.get(&key).cloned().unwrap_or(Value::Null))
                    }
                    _ => Err("Invalid index operation".to_string()),
                }
            }
            Expr::Member { obj, name, .. } => {
                let obj_val = self.eval_expr(obj)?;
                match obj_val {
                    Value::Object(map) => Ok(map.get(name).cloned().unwrap_or(Value::Null)),
                    _ => Err("Can only access properties on objects".to_string()),
                }
            }
            _ => Ok(Value::Null), // Placeholder for other expressions
        }
    }

    fn eval_binary(&self, left: Value, op: &str, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(l), Value::Int(r)) => match op {
                "+" => Ok(Value::Int(l + r)),
                "-" => Ok(Value::Int(l - r)),
                "*" => Ok(Value::Int(l * r)),
                "/" => {
                    if r == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Int(l / r))
                    }
                }
                "==" => Ok(Value::Bool(l == r)),
                "!=" => Ok(Value::Bool(l != r)),
                ">" => Ok(Value::Bool(l > r)),
                "<" => Ok(Value::Bool(l < r)),
                ">=" => Ok(Value::Bool(l >= r)),
                "<=" => Ok(Value::Bool(l <= r)),
                _ => Err(format!("Unknown operator: {}", op)),
            },
            (Value::Str(l), Value::Str(r)) if op == "+" => Ok(Value::Str(l + &r)),
            (Value::Str(l), r) if op == "+" => Ok(Value::Str(l + &r.as_string())),
            (l, Value::Str(r)) if op == "+" => Ok(Value::Str(l.as_string() + &r)),
            _ => Err(format!("Type error in binary operation: {}", op)),
        }
    }
}
