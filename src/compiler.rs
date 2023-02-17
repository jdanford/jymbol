use std::collections::HashMap;

use crate::{vm::Instruction, Result, ResultIterator, Symbol, Value, VM};

pub struct Context {
    pub params: Vec<Symbol>,
    pub locals: HashMap<Symbol, u16>,
    pub code: Vec<Instruction>,
}

impl Context {
    #[allow(clippy::cast_possible_truncation)]
    pub fn new(params: Vec<Symbol>) -> Self {
        let mut locals = HashMap::new();
        for (index, param) in params.iter().enumerate() {
            locals.insert(*param, index as u16);
        }

        Context {
            params,
            locals,
            code: Vec::new(),
        }
    }

    pub fn get(&self, sym: Symbol) -> Option<u16> {
        self.locals.get(&sym).copied()
    }

    pub fn emit(&mut self, inst: Instruction) {
        self.code.push(inst);
    }
}

pub struct Compiler<'a> {
    pub vm: &'a mut VM,
    pub contexts: Vec<Context>,
}

fn try_checked<const N: usize>(values: &[Value]) -> Result<&[Value; N]> {
    let actual_len = values.len();
    values
        .try_into()
        .map_err(|_| format!("expected {N} values, got {actual_len}"))
}

impl<'a> Compiler<'a> {
    pub fn new(vm: &'a mut VM) -> Self {
        Compiler {
            vm,
            contexts: Vec::new(),
        }
    }

    fn context(&mut self) -> Result<&mut Context> {
        self.contexts
            .last_mut()
            .ok_or_else(|| "context stack is empty".to_string())
    }

    pub fn emit(&mut self, inst: Instruction) -> Result<()> {
        let context = self.context()?;
        context.emit(inst);
        Ok(())
    }

    fn get(&self, sym: Symbol) -> Result<(u16, u16)> {
        for (n, context) in self.contexts.iter().rev().enumerate() {
            if let Some(index) = context.get(sym) {
                #[allow(clippy::cast_possible_truncation)]
                return Ok((n as u16, index));
            }
        }

        Err(format!("`{sym}` is not defined"))
    }

    fn compile(&mut self, value: &Value) -> Result<()> {
        match value {
            Value::Symbol(sym) => {
                let (frame, index) = self.get(*sym)?;
                self.emit(Instruction::Get(frame, index))?;
                Ok(())
            }
            Value::Number(num) => {
                self.emit(Instruction::Number(*num))?;
                Ok(())
            }
            Value::Compound(cons) if cons.is_cons() => {
                let (fn_value, values_list) = cons.as_cons()?;
                let values = values_list.into_iter().try_collect()?;
                self.compile_application(&fn_value, &values)
            }
            _ => Err(format!("can't compile {value}")),
        }
    }

    fn compile_application(&mut self, fn_value: &Value, values: &[Value]) -> Result<()> {
        match fn_value {
            Value::Symbol(sym) => match sym.as_str() {
                "if" => self.compile_if(values),
                "let" => self.compile_let(values),
                "fn" => self.compile_fn(values),
                "def" => self.compile_def(values),
                _ => self.compile_call(fn_value, values),
            },
            _ => Err(format!("can't apply {fn_value}")),
        }
    }

    fn compile_call(&mut self, fn_value: &Value, values: &[Value]) -> Result<()> {
        for value in values.iter() {
            self.compile(value)?;
        }

        self.compile(fn_value)?;

        let len = values.len().try_into().unwrap();
        self.emit(Instruction::Frame(len))?;
        Ok(())
    }

    fn compile_fn(&mut self, values: &[Value]) -> Result<()> {
        let [params_list, body] = try_checked(values)?;
        todo!()
    }

    fn compile_if(&mut self, values: &[Value]) -> Result<()> {
        todo!()
    }

    fn compile_let(&mut self, values: &[Value]) -> Result<()> {
        todo!()
    }

    fn compile_def(&mut self, values: &[Value]) -> Result<()> {
        let [sym_value, value] = try_checked(values)?;
        todo!()
    }
}
