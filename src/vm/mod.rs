mod frame;
mod instruction;
mod op;

pub use frame::Frame;
pub use instruction::Inst;

use im::HashMap;

use crate::{
    compiler::{context::Context, Compiler},
    function::{self, RawFn},
    Arity, Expr, FnId, Result, Symbol, Value,
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

    #[must_use]
    fn relative_frame(&mut self, frame_index: u16) -> &mut Frame {
        let max_index = self.frames.len() - 1;
        let i = max_index - frame_index as usize;
        &mut self.frames[i]
    }

    pub fn id_for_closure_type(&mut self, closure_type: &ClosureType) -> Option<FnId> {
        self.closure_ids.get(closure_type).copied()
    }

    pub fn register_closure(&mut self, closure_type: &ClosureType, code: Vec<Inst>) -> FnId {
        if let Some(&id) = self.closure_ids.get(closure_type) {
            id
        } else {
            let id = FnId::next();
            self.closure_ids.insert(closure_type.clone(), id);

            let compiled_function = function::Compiled {
                arity: Arity::Exactly(closure_type.arity),
                fn_id: id,
                code,
            };
            self.compiled_functions.insert(id, compiled_function);
            id
        }
    }

    pub fn register_native<A: Into<Arity>>(&mut self, function: RawFn, arity: A) -> FnId {
        let id = FnId::next();
        let fn_ = function::Native::new(id, function, arity);
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

    pub fn eval(&mut self, expr: &Expr) -> Result<Value> {
        let mut context = Context::new();
        let mut compiler = Compiler::new(self);
        context = compiler.compile(context, expr)?;
        let code = context.extract_code();
        let fn_id = FnId::next();
        let function = function::Compiled {
            arity: 0.into(),
            code,
            fn_id,
        };
        self.compiled_functions.insert(fn_id, function);

        let frame = Frame::compiled(fn_id, vec![]);
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

    fn step(&mut self, mut frame: frame::Compiled) -> Result<Option<Frame>> {
        let func = self.compiled_functions.get(&frame.fn_id).unwrap();
        let inst = &func.code[frame.pc as usize];
        frame.pc += 1;

        match inst {
            &Inst::Nop => {}
            &Inst::Drop => {
                self.values.pop();
            }
            Inst::Value(value) => {
                self.values.push(value.clone());
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
                let frame = self.relative_frame(frame_index);
                let locals = frame.locals();
                let value = locals[index as usize].clone();
                self.values.push(value);
            }
            &Inst::Set(frame_index, index) => {
                let value = self.pop_value();
                let frame = self.relative_frame(frame_index);
                let locals = frame.locals_mut();
                locals[index as usize] = value;
            }
            &Inst::Jump(jmp_pc) => {
                frame.pc = jmp_pc;
            }
            &Inst::JumpIf(jmp_pc) => {
                let value = self.pop_value();
                if value.is_truthy() {
                    frame.pc = jmp_pc;
                }
            }
            &Inst::JumpIfNot(jmp_pc) => {
                let value = self.pop_value();
                if !value.is_truthy() {
                    frame.pc = jmp_pc;
                }
            }
            &Inst::Call(arity) => {
                let func = self.pop_value();
                let mut locals = self.pop_values(arity as usize);
                match func {
                    Value::Closure(ref closure) => {
                        locals.extend(closure.values.clone());
                        let new_frame = Frame::compiled(closure.fn_id, locals);
                        self.frames.push(frame.into());
                        return Ok(Some(new_frame));
                    }
                    Value::NativeFunction(fn_id) => {
                        let new_frame = Frame::native(fn_id, locals);
                        self.frames.push(frame.into());
                        return Ok(Some(new_frame));
                    }
                    _ => {
                        return Err(format!("can't call {func}"));
                    }
                }
            }
            &Inst::Ret => {
                return Ok(None);
            }
        }

        Ok(Some(frame.into()))
    }
}

impl Default for VM {
    fn default() -> Self {
        VM::new()
    }
}
