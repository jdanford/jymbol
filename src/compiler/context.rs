use crate::vm::Instruction;

use super::locals::Locals;

#[derive(Clone, PartialEq, Debug)]
pub struct Base {
    locals: Locals,
    code: Vec<Instruction>,
}

impl Base {
    pub fn new() -> Self {
        Base {
            locals: Locals::new(),
            code: Vec::new(),
        }
    }

    pub fn emit(&mut self, inst: Instruction) {
        self.code.push(inst);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Extended {
    locals: Locals,
    inner: Box<Context>,
}

impl Extended {
    pub fn new(inner: Context) -> Self {
        Extended {
            locals: Locals::new(),
            inner: inner.into(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Context {
    Base(Base),
    Extended(Extended),
}

impl From<Base> for Context {
    fn from(context: Base) -> Self {
        Context::Base(context)
    }
}

impl From<Extended> for Context {
    fn from(context: Extended) -> Self {
        Context::Extended(context)
    }
}

impl Context {
    pub fn extend(self) -> Self {
        Context::Extended(Extended {
            locals: self.locals().clone(),
            inner: Box::new(self),
        })
    }

    pub fn locals(&self) -> &Locals {
        match self {
            Context::Base(context) => &context.locals,
            Context::Extended(context) => &context.locals,
        }
    }

    pub fn locals_mut(&mut self) -> &mut Locals {
        match self {
            Context::Base(context) => &mut context.locals,
            Context::Extended(context) => &mut context.locals,
        }
    }

    fn base(&mut self) -> &mut Base {
        let mut context = self;

        loop {
            match context {
                Context::Extended(extended_context) => context = &mut extended_context.inner,
                Context::Base(base_context) => return base_context,
            }
        }
    }

    pub fn emit(&mut self, inst: Instruction) {
        self.base().emit(inst);
    }
}
