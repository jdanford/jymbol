use im::{HashMap, HashSet};
use once_cell::sync::Lazy;

use crate::{symbol, Env, Expr, Result, Symbol, Value};

#[allow(clippy::module_name_repetitions)]
pub type SpecialFn = fn(&[Value]) -> Result<Expr>;

pub static FUNCTIONS: Lazy<HashMap<&str, SpecialFn>> = Lazy::new(|| {
    let mut functions: HashMap<&str, SpecialFn> = HashMap::new();

    functions.insert("do", Expr::try_from_do);
    functions.insert("fn", Expr::try_from_fn);
    functions.insert("let", Expr::try_from_let);
    functions.insert("if", Expr::try_from_if);

    functions
});

pub static VARS: Lazy<HashSet<Symbol>> = Lazy::new(|| {
    let mut vars: HashSet<Symbol> = HashSet::new();

    vars.insert(*symbol::NIL);
    vars.insert(*symbol::TRUE);
    vars.insert(*symbol::FALSE);

    for (&var, _) in (*FUNCTIONS).iter() {
        vars.insert(var.into());
    }

    vars
});

pub fn env() -> Env {
    let mut env = Env::new();

    for &var in &*VARS {
        env = env.set(var, var.into());
    }

    env
}
