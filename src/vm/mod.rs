mod frame;

pub use frame::Frame;

use im::HashMap;

use crate::{
    compiler::{context::Context, Compiler},
    function::{self, RawFn},
    Arity, Env, Expr, FnId, Inst, Result, Symbol, Value,
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
        let i = max_index - frame_index as usize;
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
        let mut context = Context::new();
        let locals = context.locals_mut();
        for (&var, _) in env.iter() {
            locals.declare(var)?;
        }

        let mut compiler = Compiler::new(self);
        context = compiler.compile(context, expr)?;
        context.code_mut().emit(Inst::Ret);
        let code = context.code_mut().extract();
        let fn_id = FnId::next();
        let function = function::Compiled::new(fn_id, 0, code);
        self.compiled_functions.insert(fn_id, function);

        let local_values = env.values().cloned().collect();
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

    fn step(&mut self, mut current_frame: frame::Compiled) -> Result<Option<Frame>> {
        let func = self.compiled_functions.get(&current_frame.fn_id).unwrap();
        let inst = &func.code[current_frame.pc as usize];
        current_frame.pc += 1;

        match inst {
            &Inst::Nop => {}
            &Inst::Drop => {
                self.values.pop();
            }
            Inst::Value(value) => {
                self.values.push(value.clone());
            }
            &Inst::List(value_count) => {
                let values = self.pop_values(value_count as usize);
                let value = Value::list(values);
                self.values.push(value);
            }
            &Inst::Compound(type_, value_count) => {
                let values = self.pop_values(value_count as usize);
                let value = Value::compound(type_, values);
                self.values.push(value);
            }
            &Inst::Closure(fn_id, value_count) => {
                let values = self.pop_values(value_count as usize);
                let value = Value::closure(fn_id, values);
                self.values.push(value);
            }
            &Inst::UnOp(op) => {
                let value = self.pop_value();
                let x: f64 = value.try_into()?;
                let y = op.apply(x);
                self.values.push(y.into());
            }
            &Inst::BinOp(op) => {
                let value_y = self.pop_value();
                let value_x = self.pop_value();
                let x: f64 = value_x.try_into()?;
                let y: f64 = value_y.try_into()?;
                let z = op.apply(x, y);
                self.values.push(z.into());
            }
            &Inst::Get(frame_index, index) => {
                let locals = if frame_index == 0 {
                    &current_frame.locals
                } else {
                    self.relative_frame(frame_index - 1).locals()
                };
                let value = locals[index as usize].clone();
                self.values.push(value);
            }
            &Inst::Set(frame_index, index) => {
                let value = self.pop_value();
                let locals = if frame_index == 0 {
                    &mut current_frame.locals
                } else {
                    self.relative_frame(frame_index - 1).locals_mut()
                };
                locals[index as usize] = value;
            }
            &Inst::Jump(jmp_pc) => {
                current_frame.pc = jmp_pc;
            }
            &Inst::JumpIf(jmp_pc) => {
                let value = self.pop_value();
                if value.is_truthy() {
                    current_frame.pc = jmp_pc;
                }
            }
            &Inst::JumpIfNot(jmp_pc) => {
                let value = self.pop_value();
                if !value.is_truthy() {
                    current_frame.pc = jmp_pc;
                }
            }
            &Inst::Call(arity) => {
                let func = self.pop_value();
                let mut locals = Vec::new();
                let new_frame = match func {
                    Value::Closure(ref closure) => {
                        locals.extend(closure.values.clone());
                        locals.extend(self.pop_values(arity as usize));
                        Ok(Frame::compiled(closure.fn_id, locals))
                    }
                    Value::NativeFunction(fn_id) => {
                        locals.extend(self.pop_values(arity as usize));
                        Ok(Frame::native(fn_id, locals))
                    }
                    _ => Err(format!("can't call {func}")),
                }?;

                self.frames.push(current_frame.into());
                return Ok(Some(new_frame));
            }
            &Inst::Ret => {
                return Ok(None);
            }
        }

        Ok(Some(current_frame.into()))
    }
}

impl Default for VM {
    fn default() -> Self {
        VM::new()
    }
}
