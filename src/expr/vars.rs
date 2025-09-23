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
            Expr::Value(_) => HashSet::new(),
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
                let mut vars = HashSet::new();
                let mut bound_vars = HashSet::new();
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
                    .sum::<HashSet<_>>()
                    + else_.free_vars()
            }
        }
    }
}
