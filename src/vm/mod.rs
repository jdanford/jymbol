mod frame;
mod instruction;
mod op;

pub use frame::{CompiledFrame, Frame, NativeFrame};
pub use instruction::Instruction;

use im::HashMap;

use crate::{function, FnId, Result, Symbol, Value};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ClosureType {
    pub arity: usize,
    pub local_params: Vec<Symbol>,
    pub captured_params: Vec<Symbol>,
    pub body: Value,
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

    fn closure_id(&mut self, closure_type: &ClosureType) -> FnId {
        if let Some(&id) = self.closure_ids.get(closure_type) {
            id
        } else {
            let id = FnId::next();
            self.closure_ids.insert(closure_type.clone(), id);
            id
        }
    }

    fn pop_value(&mut self) -> Value {
        self.values.pop().unwrap()
    }

    fn pop_values(&mut self, count: usize) -> Vec<Value> {
        let len = self.values.len();
        let i = len - count;
        self.values.split_off(i)
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

    fn step(&mut self, mut frame: CompiledFrame) -> Result<Option<Frame>> {
        let func = self.compiled_functions.get(&frame.fn_id).unwrap();
        let inst = &func.code[frame.pc as usize];
        frame.pc += 1;

        match inst {
            &Instruction::Nop => {}
            &Instruction::Drop => {
                self.values.pop();
            }
            Instruction::Value(value) => {
                self.values.push(value.clone());
            }
            &Instruction::Compound(type_, value_count) => {
                let values = self.pop_values(value_count as usize);
                let value = Value::compound(type_, values);
                self.values.push(value);
            }
            &Instruction::Closure(fn_id, value_count) => {
                let values = self.pop_values(value_count as usize);
                let value = Value::closure(fn_id, values);
                self.values.push(value);
            }
            &Instruction::UnOp(op) => {
                let value = self.pop_value();
                let x: f64 = value.try_into()?;
                let y = op.apply(x);
                self.values.push(y.into());
            }
            &Instruction::BinOp(op) => {
                let value_y = self.pop_value();
                let value_x = self.pop_value();
                let x: f64 = value_x.try_into()?;
                let y: f64 = value_y.try_into()?;
                let z = op.apply(x, y);
                self.values.push(z.into());
            }
            &Instruction::Get(frame_index, index) => {
                let frame = self.relative_frame(frame_index);
                let locals = frame.locals();
                let value = locals[index as usize].clone();
                self.values.push(value);
            }
            &Instruction::Set(frame_index, index) => {
                let value = self.pop_value();
                let frame = self.relative_frame(frame_index);
                let locals = frame.locals();
                locals[index as usize] = value;
            }
            &Instruction::Jump(jmp_pc) => {
                frame.pc = jmp_pc;
            }
            &Instruction::Branch(true_pc, false_pc) => {
                let value = self.pop_value();
                if value.is_truthy() {
                    frame.pc = true_pc;
                } else {
                    frame.pc = false_pc;
                }
            }
            &Instruction::Call(arity) => {
                let func = self.pop_value();
                let locals = self.pop_values(arity as usize);
                match func {
                    Value::Closure(ref closure) => {
                        let new_frame = Frame::Compiled(CompiledFrame {
                            fn_id: closure.fn_id,
                            locals,
                            pc: 0,
                        });
                        self.frames.push(frame.into());
                        return Ok(Some(new_frame));
                    }
                    Value::NativeFunction(fn_id) => {
                        let new_frame = Frame::Native(NativeFrame { fn_id, locals });
                        self.frames.push(frame.into());
                        return Ok(Some(new_frame));
                    }
                    _ => {
                        return Err(format!("can't call {func}"));
                    }
                }
            }
            &Instruction::Ret => {
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
