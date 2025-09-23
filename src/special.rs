use std::{collections::HashMap, sync::LazyLock};

use im::{OrdSet};

use crate::{
    op::{Binary, Unary},
    symbol, Expr, Result, Symbol, Value,
};

#[allow(clippy::module_name_repetitions)]
pub type SpecialFn = fn(&[Value]) -> Result<Expr>;

pub static FUNCTIONS: LazyLock<HashMap<Symbol, SpecialFn>> = LazyLock::new(|| {
    let mut functions: HashMap<Symbol, SpecialFn> = HashMap::new();

    functions.insert("do".into(), Expr::try_from_do);
    functions.insert("fn".into(), Expr::try_from_fn);
    functions.insert("let".into(), Expr::try_from_let);
    functions.insert("if".into(), Expr::try_from_if);

    functions.insert("$abs".into(), |values| Expr::try_from_unop(Unary::Abs, values));
    functions.insert("$neg".into(), |values| Expr::try_from_unop(Unary::Neg, values));
    functions.insert("$sqrt".into(), |values| Expr::try_from_unop(Unary::Sqrt, values));
    functions.insert("$trunc".into(), |values| Expr::try_from_unop(Unary::Trunc, values));
    functions.insert("$fract".into(), |values| Expr::try_from_unop(Unary::Fract, values));
    functions.insert("$round".into(), |values| Expr::try_from_unop(Unary::Round, values));
    functions.insert("$floor".into(), |values| Expr::try_from_unop(Unary::Floor, values));
    functions.insert("$ceil".into(), |values| Expr::try_from_unop(Unary::Ceil, values));
    functions.insert("$not".into(), |values| Expr::try_from_unop(Unary::Not, values));

    functions.insert("$add".into(), |values| Expr::try_from_binop(Binary::Add, values));
    functions.insert("$sub".into(), |values| Expr::try_from_binop(Binary::Sub, values));
    functions.insert("$mul".into(), |values| Expr::try_from_binop(Binary::Mul, values));
    functions.insert("$div".into(), |values| Expr::try_from_binop(Binary::Div, values));
    functions.insert("$mod".into(), |values| Expr::try_from_binop(Binary::Mod, values));
    functions.insert("$pow".into(), |values| Expr::try_from_binop(Binary::Pow, values));
    functions.insert("$shl".into(), |values| Expr::try_from_binop(Binary::Shl, values));
    functions.insert("$shr".into(), |values| Expr::try_from_binop(Binary::Shr, values));
    functions.insert("$and".into(), |values| Expr::try_from_binop(Binary::And, values));
    functions.insert("$or".into(), |values| Expr::try_from_binop(Binary::Or, values));
    functions.insert("$xor".into(), |values| Expr::try_from_binop(Binary::Xor, values));
    functions.insert("$eq".into(), |values| Expr::try_from_binop(Binary::Eq, values));
    functions.insert("$ne".into(), |values| Expr::try_from_binop(Binary::Ne, values));
    functions.insert("$lt".into(), |values| Expr::try_from_binop(Binary::Lt, values));
    functions.insert("$gt".into(), |values| Expr::try_from_binop(Binary::Gt, values));
    functions.insert("$le".into(), |values| Expr::try_from_binop(Binary::Le, values));
    functions.insert("$ge".into(), |values| Expr::try_from_binop(Binary::Ge, values));

    functions
});

pub static VARS: LazyLock<OrdSet<Symbol>> = LazyLock::new(|| {
    let mut vars: OrdSet<Symbol> = OrdSet::new();

    vars.insert(*symbol::NIL);
    vars.insert(*symbol::TRUE);
    vars.insert(*symbol::FALSE);

    for &var in FUNCTIONS.keys() {
        vars.insert(var);
    }

    vars
});
