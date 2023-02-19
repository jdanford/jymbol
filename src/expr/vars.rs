use im::HashSet;

use crate::{Expr, Symbol};

pub fn single(var: Symbol) -> HashSet<Symbol> {
    let mut vars = HashSet::new();
    vars.insert(var);
    vars
}

pub fn list(vars: &[Symbol]) -> HashSet<Symbol> {
    vars.iter().copied().collect()
}

impl Expr {
    pub fn free_vars(&self) -> HashSet<Symbol> {
        match self {
            &Expr::Var(var) => single(var),
            Expr::List(exprs) => exprs.iter().map(Expr::free_vars).sum(),
            Expr::Call { args, .. } => args.iter().map(Expr::free_vars).sum(),
            Expr::Fn { params, body } => {
                let body_vars = body.free_vars();
                let bound_vars = list(params);
                body_vars.difference(bound_vars)
            }
            Expr::Let { var, value, body } => {
                let vars = body.free_vars() + value.free_vars();
                let bound_vars = single(*var);
                vars.difference(bound_vars)
            }
            Expr::If { cond, then, else_ } => {
                cond.free_vars() + then.free_vars() + else_.free_vars()
            }
            Expr::Value(_) => HashSet::new(),
        }
    }
}
