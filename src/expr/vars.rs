use im::OrdSet;

use crate::{Expr, Symbol};

pub fn single(var: Symbol) -> OrdSet<Symbol> {
    let mut vars = OrdSet::new();
    vars.insert(var);
    vars
}

pub fn list(vars: &[Symbol]) -> OrdSet<Symbol> {
    vars.iter().copied().collect()
}

impl Expr {
    pub fn free_vars(&self) -> OrdSet<Symbol> {
        match self {
            Expr::Value(_) => OrdSet::new(),
            &Expr::Var(var) => single(var),
            Expr::List(exprs) | Expr::Do(exprs) => exprs.iter().map(Expr::free_vars).sum(),
            Expr::Call { fn_, args } => fn_.free_vars() + args.iter().map(Expr::free_vars).sum(),
            Expr::UnOp { expr, .. } => expr.free_vars(),
            Expr::BinOp { left, right, .. } => left.free_vars() + right.free_vars(),
            Expr::Fn { params, body } => {
                let body_vars = body.free_vars();
                let bound_vars = list(params);
                body_vars.difference(bound_vars)
            }
            Expr::Let {
                var_expr_pairs,
                body,
            } => {
                let mut vars = OrdSet::new();
                let mut bound_vars = OrdSet::new();
                for (var, expr) in var_expr_pairs {
                    bound_vars.insert(*var);
                    let expr_vars = expr.free_vars().relative_complement(bound_vars.clone());
                    vars.extend(expr_vars);
                }

                let body_vars = body.free_vars().relative_complement(bound_vars);
                vars.extend(body_vars);
                vars
            }
            Expr::If {
                cond_expr_pairs,
                else_,
            } => {
                cond_expr_pairs
                    .iter()
                    .map(|(cond, expr)| cond.free_vars() + expr.free_vars())
                    .sum::<OrdSet<_>>()
                    + else_.free_vars()
            }
        }
    }
}
