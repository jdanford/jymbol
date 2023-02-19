use crate::{Inst, Result, Value, VM};

use super::{frame, Frame};

impl VM {
    pub(crate) fn step(&mut self, mut current_frame: frame::Compiled) -> Result<Option<Frame>> {
        let func = self.compiled_functions.get(&current_frame.fn_id).unwrap();
        let inst = &func.code[current_frame.pc as usize];
        current_frame.pc += 1;

        match inst {
            Inst::Nop => {}
            Inst::Drop => {
                self.values.pop();
            }
            Inst::Value(ref value) => {
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
                    self.relative_frame(frame_index).locals()
                };
                let value = locals[index as usize].clone();
                self.values.push(value);
            }
            &Inst::Set(frame_index, index) => {
                let value = self.pop_value();
                let locals = if frame_index == 0 {
                    &mut current_frame.locals
                } else {
                    self.relative_frame(frame_index).locals_mut()
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
                let new_frame = self.frame_from_func(&func, arity)?;
                self.frames.push(current_frame.into());
                return Ok(Some(new_frame));
            }
            Inst::Ret => {
                return Ok(None);
            }
        }

        Ok(Some(current_frame.into()))
    }
}
