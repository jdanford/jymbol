use std::ops::{Deref, DerefMut};

use crate::vm::Inst;

use super::locals::Locals;

#[derive(Clone, PartialEq, Debug)]
pub struct Base {
    locals: Locals,
    code: Vec<Inst>,
}

impl Base {
    pub fn new() -> Self {
        Base {
            locals: Locals::new(),
            code: Vec::new(),
        }
    }

    pub fn pc(&self) -> u32 {
        u32::try_from(self.code.len()).expect("pc is out of range")
    }

    pub fn emit(&mut self, inst: Inst) {
        self.code.push(inst);
    }

    pub fn update(&mut self, pc: u32, inst: Inst) {
        self.code[pc as usize] = inst;
    }

    pub fn bookmark(&mut self) -> u32 {
        let pc = self.pc();
        self.emit(Inst::Nop);
        pc
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

    fn base(&self) -> &Base {
        let mut context = self;

        loop {
            match context {
                Context::Extended(extended_context) => context = &extended_context.inner,
                Context::Base(base_context) => return base_context,
            }
        }
    }

    fn base_mut(&mut self) -> &mut Base {
        let mut context = self;

        loop {
            match context {
                Context::Extended(extended_context) => context = &mut extended_context.inner,
                Context::Base(base_context) => return base_context,
            }
        }
    }
}

impl Deref for Context {
    type Target = Base;

    fn deref(&self) -> &Self::Target {
        self.base()
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.base_mut()
    }
}
