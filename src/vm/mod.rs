mod frame;
mod step;

use anyhow::anyhow;
pub use frame::Frame;

use intmap::IntMap;

use crate::{
    Arity, Env, Expr, FnId, Inst, Result, ResultIterator, Value,
    compiler::{Compiler, context::Context},
    function::{self, RawFn},
};

#[derive(Debug)]
pub struct VM {
    frames: Vec<Frame>,
    values: Vec<Value>,
    compiled_functions: IntMap<FnId, function::Compiled>,
    next_compiled_fn_id: FnId,
    native_functions: IntMap<FnId, function::Native>,
    next_native_fn_id: FnId,
}

impl VM {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new() -> Self {
        VM {
            frames: Vec::new(),
            values: Vec::new(),
            compiled_functions: IntMap::new(),
            next_compiled_fn_id: 0,
            native_functions: IntMap::new(),
            next_native_fn_id: 0,
        }
    }

    fn relative_frame(&mut self, frame_index: u16) -> &mut Frame {
        let max_index = self.frames.len() - 1;
        let i = max_index - frame_index as usize - 1;
        &mut self.frames[i]
    }

    pub fn register_closure<A: Into<Arity>>(&mut self, arity: A, code: Vec<Inst>) -> FnId {
        let id = self.next_compiled_fn_id;
        self.next_compiled_fn_id += 1;
        let compiled_function = function::Compiled::new(id, arity, code);
        self.compiled_functions.insert(id, compiled_function);
        id
    }

    pub fn register_native<A: Into<Arity>>(&mut self, function: RawFn, arity: A) -> FnId {
        let id = self.next_native_fn_id;
        self.next_native_fn_id += 1;
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

        let mut context = Context::blank();
        let locals = &mut context.locals;
        for &var in &free_vars {
            locals.declare(var)?;
        }

        let mut compiler = Compiler::new(self);
        context = compiler.compile(context, expr)?;
        context.code.emit(Inst::Return);

        let code = context.code.extract();
        let fn_id = self.next_compiled_fn_id;
        let function = function::Compiled::new(fn_id, 0, code);
        self.compiled_functions.insert(fn_id, function);

        let local_values = free_vars.iter().map(|&var| env.get(var)).try_collect()?;
        let frame = Frame::compiled(fn_id, local_values);
        self.frames.push(frame);

        let run_result = self.run();
        self.compiled_functions.remove(fn_id);
        run_result?;

        let value = self.pop_value();
        Ok(value)
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
                    let func = self.native_functions.get(native_frame.fn_id).unwrap();
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
            _ => Err(anyhow!("can't call {func}")),
        }
    }

    fn compiled_frame(&mut self, closure: &function::Closure, arity: u16) -> Frame {
        let mut locals = Vec::new();
        locals.extend(closure.values.clone());
        locals.extend(self.pop_values(arity.into()));
        Frame::compiled(closure.fn_id, locals)
    }

    fn native_frame(&mut self, fn_id: FnId, arity: u16) -> Frame {
        let mut locals = Vec::new();
        locals.extend(self.pop_values(arity.into()));
        Frame::native(fn_id, locals)
    }
}

impl Default for VM {
    fn default() -> Self {
        VM::new()
    }
}
