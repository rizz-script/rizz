use crate::ast::{Expr, Program, Stmt, TypeName};
use crate::types::{is_assignable, parse_type, TypeId};

#[derive(Default)]
pub struct TypeEnv {
    scopes: Vec<std::collections::HashMap<String, TypeId>>,
}

impl TypeEnv {
    fn new() -> Self {
        Self {
            scopes: vec![std::collections::HashMap::new()],
        }
    }

    fn push(&mut self) {
        self.scopes.push(std::collections::HashMap::new());
    }

    fn pop(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    fn set(&mut self, name: &str, ty: TypeId) {
        if let Some(s) = self.scopes.last_mut() {
            s.insert(name.to_string(), ty);
        }
    }

    fn get(&self, name: &str) -> Option<TypeId> {
        for s in self.scopes.iter().rev() {
            if let Some(t) = s.get(name) {
                return Some(t.clone());
            }
        }
        None
    }
}

pub fn typecheck(program: &Program) -> anyhow::Result<()> {
    let mut env = TypeEnv::new();
    for s in &program.statements {
        check_stmt(s, &mut env)?;
    }
    Ok(())
}

fn tn(t: &TypeName) -> TypeId {
    parse_type(&t.name)
}

fn check_stmt(s: &Stmt, env: &mut TypeEnv) -> anyhow::Result<()> {
    match s {
        Stmt::VarDecl { name, ty, value, .. } | Stmt::ConstDecl { name, ty, value, .. } => {
            let inferred = infer_expr(value, env);
            if let Some(t) = ty {
                let want = tn(t);
                if !is_assignable(&want, &inferred) {
                    anyhow::bail!("Type error: {} annotated as {:?} but assigned {:?}", name, want, inferred);
                }
                env.set(name, want);
            } else {
                env.set(name, inferred);
            }
        }
        Stmt::Assign { name, value, .. } => {
            let rhs = infer_expr(value, env);
            if let Some(lhs) = env.get(name) {
                if !is_assignable(&lhs, &rhs) {
                    anyhow::bail!("Type error: assigning {:?} to {:?} variable {}", rhs, lhs, name);
                }
            }
        }
        Stmt::FuncDef { name, params, ret_ty, .. } => {
            // store function type as "function" (no full signatures yet)
            env.set(name, TypeId::Function);
            env.push();
            for p in params {
                if let Some(t) = &p.ty {
                    env.set(&p.name, tn(t));
                } else {
                    env.set(&p.name, TypeId::Any);
                }
            }
            // return type: if annotated, record in env under special key
            if let Some(rt) = ret_ty {
                env.set("__return", tn(rt));
            }
            env.pop();
        }
        Stmt::Try { try_block, catch_name, catch_block, finally_block, .. } => {
            env.push();
            for st in &try_block.statements {
                check_stmt(st, env)?;
            }
            env.pop();
            if let Some(cb) = catch_block {
                env.push();
                if let Some(n) = catch_name {
                    env.set(n, TypeId::Any);
                }
                for st in &cb.statements {
                    check_stmt(st, env)?;
                }
                env.pop();
            }
            if let Some(fb) = finally_block {
                env.push();
                for st in &fb.statements {
                    check_stmt(st, env)?;
                }
                env.pop();
            }
        }
        _ => {}
    }
    Ok(())
}

fn infer_expr(e: &Expr, env: &mut TypeEnv) -> TypeId {
    match e {
        Expr::Literal { lit, .. } => match lit {
            crate::ast::Lit::Null => TypeId::Null,
            crate::ast::Lit::Bool(_) => TypeId::Bool,
            crate::ast::Lit::Int(_) => TypeId::Int,
            crate::ast::Lit::Float(_) => TypeId::Float,
            crate::ast::Lit::Char(_) => TypeId::String,
            crate::ast::Lit::Str(_) => TypeId::String,
            crate::ast::Lit::Regex(_) => TypeId::Regex,
        },
        Expr::Array { .. } => TypeId::Array,
        Expr::Object { .. } => TypeId::Object,
        Expr::Ident { name, .. } => env.get(name).unwrap_or(TypeId::Any),
        Expr::Call { .. } => TypeId::Any,
        Expr::Binary { .. } => TypeId::Any,
        Expr::Unary { .. } => TypeId::Any,
        Expr::Ternary { .. } => TypeId::Any,
        Expr::Member { .. } => TypeId::Any,
        Expr::Index { .. } => TypeId::Any,
        Expr::Vibe { .. } => TypeId::Task,
        Expr::Attempt { .. } => TypeId::Any,
        Expr::Range { .. } => TypeId::Array,
    }
}

