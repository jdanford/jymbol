use im::{HashMap, OrdSet};
use once_cell::sync::Lazy;

use crate::{
    op::{Binary, Unary},
    symbol, Expr, Result, Symbol, Value,
};

#[allow(clippy::module_name_repetitions)]
pub type SpecialFn = fn(&[Value]) -> Result<Expr>;

pub static FUNCTIONS: Lazy<HashMap<&str, SpecialFn>> = Lazy::new(|| {
    let mut functions: HashMap<&str, SpecialFn> = HashMap::new();

    functions.insert("do", Expr::try_from_do);
    functions.insert("fn", Expr::try_from_fn);
    functions.insert("let", Expr::try_from_let);
    functions.insert("if", Expr::try_from_if);

    functions.insert("$abs", |values| Expr::try_from_unop(Unary::Abs, values));
    functions.insert("$neg", |values| Expr::try_from_unop(Unary::Neg, values));
    functions.insert("$sqrt", |values| Expr::try_from_unop(Unary::Sqrt, values));
    functions.insert("$trunc", |values| Expr::try_from_unop(Unary::Trunc, values));
    functions.insert("$fract", |values| Expr::try_from_unop(Unary::Fract, values));
    functions.insert("$round", |values| Expr::try_from_unop(Unary::Round, values));
    functions.insert("$floor", |values| Expr::try_from_unop(Unary::Floor, values));
    functions.insert("$ceil", |values| Expr::try_from_unop(Unary::Ceil, values));
    functions.insert("$not", |values| Expr::try_from_unop(Unary::Not, values));

    functions.insert("$add", |values| Expr::try_from_binop(Binary::Add, values));
    functions.insert("$sub", |values| Expr::try_from_binop(Binary::Sub, values));
    functions.insert("$mul", |values| Expr::try_from_binop(Binary::Mul, values));
    functions.insert("$div", |values| Expr::try_from_binop(Binary::Div, values));
    functions.insert("$mod", |values| Expr::try_from_binop(Binary::Mod, values));
    functions.insert("$pow", |values| Expr::try_from_binop(Binary::Pow, values));
    functions.insert("$shl", |values| Expr::try_from_binop(Binary::Shl, values));
    functions.insert("$shr", |values| Expr::try_from_binop(Binary::Shr, values));
    functions.insert("$and", |values| Expr::try_from_binop(Binary::And, values));
    functions.insert("$or", |values| Expr::try_from_binop(Binary::Or, values));
    functions.insert("$xor", |values| Expr::try_from_binop(Binary::Xor, values));
    functions.insert("$eq", |values| Expr::try_from_binop(Binary::Eq, values));
    functions.insert("$ne", |values| Expr::try_from_binop(Binary::Ne, values));
    functions.insert("$lt", |values| Expr::try_from_binop(Binary::Lt, values));
    functions.insert("$gt", |values| Expr::try_from_binop(Binary::Gt, values));
    functions.insert("$le", |values| Expr::try_from_binop(Binary::Le, values));
    functions.insert("$ge", |values| Expr::try_from_binop(Binary::Ge, values));

    functions
});

pub static VARS: Lazy<OrdSet<Symbol>> = Lazy::new(|| {
    let mut vars: OrdSet<Symbol> = OrdSet::new();

    vars.insert(*symbol::NIL);
    vars.insert(*symbol::TRUE);
    vars.insert(*symbol::FALSE);

    for (&var, _) in &*FUNCTIONS {
        vars.insert(var.into());
    }

    vars
});
