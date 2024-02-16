mod from;
pub mod vars;

use crate::{op, Symbol, Value};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Expr {
    Value(Value),
    Var(Symbol),
    List(Vec<Expr>),
    Do(Vec<Expr>),
    UnOp {
        op: op::Unary,
        expr: Box<Expr>,
    },
    BinOp {
        op: op::Binary,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        fn_: Box<Expr>,
        args: Vec<Expr>,
    },
    Fn {
        params: Vec<Symbol>,
        body: Box<Expr>,
    },
    Let {
        var_expr_pairs: Vec<(Symbol, Expr)>,
        body: Box<Expr>,
    },
    If {
        cond_expr_pairs: Vec<(Expr, Expr)>,
        else_: Box<Expr>,
    },
    // Loop {
    //     values: Vec<Expr>,
    //     body: Box<Expr>,
    // },
    // Recur {
    //     values: Vec<Expr>,
    // },
}
