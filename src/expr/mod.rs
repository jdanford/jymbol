mod from;
pub mod vars;

use crate::{Symbol, Value};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Expr {
    Value(Value),
    Var(Symbol),
    List(Vec<Expr>),
    Call {
        fn_: Box<Expr>,
        args: Vec<Expr>,
    },
    Fn {
        params: Vec<Symbol>,
        body: Box<Expr>,
    },
    Let {
        var: Symbol,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    // Loop {
    //     values: Vec<Expr>,
    //     body: Box<Expr>,
    // },
    // Recur {
    //     values: Vec<Expr>,
    // },
    // Do {
    //     exprs: Vec<Expr>,
    // },
}
