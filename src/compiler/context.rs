use std::ops::{Deref, DerefMut};

use super::{code::Code, locals::Locals};

#[derive(Clone, PartialEq, Debug)]
pub struct Base {
    locals: Locals,
    code: Code,
}

impl Base {
    pub fn new() -> Self {
        Base {
            locals: Locals::new(),
            code: Code::new(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Extended {
    locals: Locals,
    inner: Box<Context>,
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
    pub fn new() -> Self {
        Base::new().into()
    }

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

    pub fn code(&self) -> &Code {
        &self.base().code
    }

    pub fn code_mut(&mut self) -> &mut Code {
        &mut self.base_mut().code
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
