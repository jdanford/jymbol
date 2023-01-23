use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use crate::{
    check_count, hash::ValueHasher, symbol, Env, Heap, ListIterator, Ref, Result, Symbol, Value,
};

pub struct VM {
    pub heap: Heap,
}

impl VM {
    #[must_use]
    pub fn new() -> Self {
        VM { heap: Heap::new() }
    }

    pub fn hash(&self, value: &Value) -> Result<u32> {
        let inner_hasher = DefaultHasher::new();
        let mut hasher = ValueHasher::new(inner_hasher, self);

        hasher.write_value(value)?;
        let full_hash = hasher.finish();

        #[allow(clippy::cast_possible_truncation)]
        let hash = full_hash as u32;
        Ok(hash)
    }

    pub fn iter_list(&mut self, value: Value) -> ListIterator<'_> {
        ListIterator::new(self, value)
    }

    pub fn eval(&mut self, env: &Env, value: Value) -> Result<Value> {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match value {
            Value::Symbol(symbol) => self.eval_symbol(env, symbol),
            Value::Ref(ref_) => self.eval_ref(env, ref_),
            _ => Ok(value),
        }
    }

    #[allow(clippy::unused_self)]
    fn eval_symbol(&self, env: &Env, symbol: Symbol) -> Result<Value> {
        env.get(symbol)
            .ok_or_else(|| format!("undefined symbol: {}", symbol))
    }

    fn eval_ref(&mut self, env: &Env, ref_: Ref) -> Result<Value> {
        let (fn_, args) = self.heap.load_cons(ref_)?;
        self.apply(env, fn_, args)
    }

    fn apply(&mut self, env: &Env, func: Value, args: Value) -> Result<Value> {
        let ref_: Ref = func.try_into()?;
        let (type_, values) = self.heap.load(ref_)?;

        if type_ == *symbol::FN {
            check_count(*symbol::FN, values, 2)?;
            let params = values[0];
            let body = values[1];
            self.apply_fn(env, params, args, body)
        } else {
            Err(format!("can't apply {}", type_))
        }
    }

    fn apply_fn(&mut self, env: &Env, params: Value, args: Value, body: Value) -> Result<Value> {
        let param_results: Vec<Result<Value>> = self.iter_list(params).collect();
        let arg_results: Vec<Result<Value>> = self.iter_list(args).collect();

        let expected_arity = param_results.len();
        let actual_arity = arg_results.len();
        if expected_arity != actual_arity {
            return Err(format!(
                "expected {} arguments, got {}",
                expected_arity, actual_arity
            ));
        }

        let mut new_env = env.clone();
        for (param_result, arg_result) in param_results.into_iter().zip(arg_results) {
            let param = param_result?;
            let arg = arg_result?;
            if let Value::Symbol(param_symbol) = param {
                let value = self.eval(env, arg)?;
                new_env = new_env.update(param_symbol, value);
            }
        }

        self.eval(&new_env, body)
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
