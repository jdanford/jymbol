use crate::{Inst, Result, Value, VM};

use super::{frame, Frame};

impl VM {
    pub(crate) fn step(&mut self, mut current_frame: frame::Compiled) -> Result<Option<Frame>> {
        let func = self.compiled_functions.get(current_frame.fn_id).unwrap();
        let inst = &func.code[current_frame.pc as usize];
        current_frame.pc += 1;

        match inst {
            Inst::Nop => {}
            Inst::Drop => {
                self.values.pop();
            }
            Inst::Value(value) => {
                self.values.push(value.clone());
            }
            &Inst::List(value_count) => {
                let values = self.pop_values(value_count.into());
                let value = Value::list(values);
                self.values.push(value);
            }
            &Inst::Compound(type_, value_count) => {
                let values = self.pop_values(value_count.into());
                let value = Value::compound(type_, values);
                self.values.push(value);
            }
            &Inst::Closure(fn_id, value_count) => {
                let values = self.pop_values(value_count.into());
                let value = Value::closure(fn_id, values);
                self.values.push(value);
            }
            &Inst::UnOp(op) => {
                let a = self.pop_value();
                let b = op.apply(&a)?;
                self.values.push(b);
            }
            &Inst::BinOp(op) => {
                let b = self.pop_value();
                let a = self.pop_value();
                let c = op.apply(&a, &b)?;
                self.values.push(c);
            }
            &Inst::Get(frame_index, index) => {
                let locals = if frame_index == 0 {
                    &current_frame.locals
                } else {
                    self.relative_frame(frame_index).locals()
                };
                let value = locals[index as usize].clone();
                self.values.push(value);
            }
            &Inst::Set(frame_index, index) => {
                let index_ = usize::from(index);
                let value = self.pop_value();
                let locals = if frame_index == 0 {
                    &mut current_frame.locals
                } else {
                    self.relative_frame(frame_index).locals_mut()
                };

                if index_ >= locals.len() {
                    let new_len = index_ + 1;
                    locals.resize(new_len, Value::nil());
                }

                locals[index_] = value;
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
                let new_frame = self.frame_from_func(&func, arity)?;
                self.frames.push(current_frame.into());
                return Ok(Some(new_frame));
            }
            Inst::Return => {
                return Ok(None);
            }
        }

        Ok(Some(current_frame.into()))
    }
}
