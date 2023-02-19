mod frame;
mod step;

pub use frame::Frame;

use im::HashMap;

use crate::{
    compiler::{context::Context, Compiler},
    function::{self, RawFn},
    Arity, Env, Expr, FnId, Inst, Result, ResultIterator, Symbol, Value,
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ClosureType {
    pub arity: usize,
    pub local_params: Vec<Symbol>,
    pub captured_params: Vec<Symbol>,
    pub body: Expr,
}

pub struct VM {
    frames: Vec<Frame>,
    values: Vec<Value>,
    closure_ids: HashMap<ClosureType, FnId>,
    compiled_functions: HashMap<FnId, function::Compiled>,
    native_functions: HashMap<FnId, function::Native>,
}

impl VM {
    #[must_use]
    pub fn new() -> Self {
        VM {
            frames: Vec::new(),
            values: Vec::new(),
            closure_ids: HashMap::new(),
            compiled_functions: HashMap::new(),
            native_functions: HashMap::new(),
        }
    }

    fn relative_frame(&mut self, frame_index: u16) -> &mut Frame {
        let max_index = self.frames.len() - 1;
        let i = max_index - frame_index as usize - 1;
        &mut self.frames[i]
    }

    pub fn id_for_closure_type(&mut self, closure_type: &ClosureType) -> Option<FnId> {
        self.closure_ids.get(closure_type).copied()
    }

    pub fn register_closure(&mut self, closure_type: &ClosureType, code: Vec<Inst>) -> FnId {
        if let Some(id) = self.id_for_closure_type(closure_type) {
            return id;
        }

        let id = FnId::next();
        self.closure_ids.insert(closure_type.clone(), id);

        let compiled_function = function::Compiled::new(id, closure_type.arity, code);
        self.compiled_functions.insert(id, compiled_function);
        id
    }

    pub fn register_native<A: Into<Arity>>(&mut self, function: RawFn, arity: A) -> FnId {
        let id = FnId::next();
        let fn_ = function::Native::new(id, arity, function);
        self.native_functions.insert(id, fn_);
        id
    }

    fn pop_value(&mut self) -> Value {
        self.values.pop().unwrap()
    }

    fn pop_values(&mut self, count: usize) -> Vec<Value> {
        let len = self.values.len();
        let i = len - count;
        self.values.split_off(i)
    }

    pub fn eval(&mut self, env: &Env, expr: &Expr) -> Result<Value> {
        let free_vars = expr.free_vars();

        let mut context = Context::new();
        let locals = context.locals_mut();
        for &var in free_vars.iter() {
            locals.declare(var)?;
        }

        let mut compiler = Compiler::new(self);
        context = compiler.compile(context, expr)?;
        context.code_mut().emit(Inst::Ret);

        let code = context.code_mut().extract();
        let fn_id = FnId::next();
        let function = function::Compiled::new(fn_id, 0, code);
        self.compiled_functions.insert(fn_id, function);

        let local_values = free_vars.iter().map(|&var| env.get(var)).try_collect()?;
        let frame = Frame::compiled(fn_id, local_values);
        self.frames.push(frame);
        self.run()?;

        self.values
            .pop()
            .ok_or_else(|| "stack is empty".to_string())
    }

    fn run(&mut self) -> Result<()> {
        while let Some(frame) = self.frames.pop() {
            match frame {
                Frame::Compiled(compiled_frame) => {
                    if let Some(frame) = self.step(compiled_frame)? {
                        self.frames.push(frame);
                    }
                }
                Frame::Native(native_frame) => {
                    let func = self.native_functions.get(&native_frame.fn_id).unwrap();
                    let value = func.apply(&native_frame.locals)?;
                    self.values.push(value);
                }
            }
        }

        Ok(())
    }

    fn frame_from_func(&mut self, func: &Value, arity: u16) -> Result<Frame> {
        match func {
            Value::Closure(closure) => Ok(self.compiled_frame(closure, arity)),
            &Value::NativeFunction(fn_id) => Ok(self.native_frame(fn_id, arity)),
            _ => Err(format!("can't call {func}")),
        }
    }

    fn compiled_frame(&mut self, closure: &function::Closure, arity: u16) -> Frame {
        let mut locals = Vec::new();
        locals.extend(closure.values.clone());
        locals.extend(self.pop_values(arity as usize));
        Frame::compiled(closure.fn_id, locals)
    }

    fn native_frame(&mut self, fn_id: FnId, arity: u16) -> Frame {
        let mut locals = Vec::new();
        locals.extend(self.pop_values(arity as usize));
        Frame::native(fn_id, locals)
    }
}

impl Default for VM {
    fn default() -> Self {
        VM::new()
    }
}
